import { app, BrowserWindow, dialog, ipcMain } from 'electron';
import dnssd from 'dnssd';
import { IPC_MESSAGES } from './shared';
import { address as getIpAddress, isV4Format } from 'ip';
import {
  FrontEndHost,
  frontEndHostUrl,
  isProtocol,
} from '@openmsupply-client/common/src/hooks/useNativeClient';
import HID from 'node-hid';
import ElectronStore from 'electron-store';

const SERVICE_TYPE = 'omsupply';
const PROTOCOL_KEY = 'protocol';
const CLIENT_VERSION_KEY = 'client_version';
const HARDWARE_ID_KEY = 'hardware_id';
const SUPPORTED_SCANNERS = [
  {
    vendorId: 1504,
    vendorName: 'Zebra / Symbol Technologies, Inc, 2008',
    products: [
      { id: 2194, model: 'DS2208' },
      { id: 4864, model: 'DS2208' },
    ],
  },
];

class BarcodeScanner {
  device: HID.HID | undefined;

  constructor() {
    this.device = this.findDevice();
  }

  private findDevice() {
    const devices = HID.devices();
    for (const scanner of SUPPORTED_SCANNERS) {
      // const productIds = scanner.products.map(p => p.id);
      const deviceInfo = devices.find(
        d => d.vendorId === scanner.vendorId // &&
        // productIds.some(pid => d.productId === pid);
      );

      if (deviceInfo && !!deviceInfo.path) {
        return new HID.HID(deviceInfo.path);
      }
    }
    return undefined;
  }

  start(window: BrowserWindow) {
    if (!this.device) throw new Error('No scanners found');
    this.device?.on('data', data => {
      window.webContents.send(IPC_MESSAGES.ON_BARCODE_SCAN, data);
    });
  }

  stop() {
    try {
      this.device?.close();
      this.device = this.findDevice();
    } catch {}
  }
}

const discovery = new dnssd.Browser(dnssd.tcp(SERVICE_TYPE));
const barcodeScanner = new BarcodeScanner();

let connectedServer: FrontEndHost | null = null;
let discoveredServers: FrontEndHost[] = [];
let hasLoadingError = false;

// This allows TypeScript to pick up the magic constant that's auto-generated by Forge's Webpack
// plugin that tells the Electron app where to look for the Webpack-bundled app code (depending on
// whether you're running in development or production).
declare const MAIN_WINDOW_WEBPACK_ENTRY: string;
declare const MAIN_WINDOW_PRELOAD_WEBPACK_ENTRY: string;

const getDebugHost = () => {
  const { ELECTRON_HOST } = process.env;
  return (typeof ELECTRON_HOST !== 'undefined' && ELECTRON_HOST) || '';
};

// Can debug by opening chrome chrome://inspect and open inspect under 'devices'
const START_URL = getDebugHost()
  ? `${getDebugHost()}/discovery`
  : MAIN_WINDOW_WEBPACK_ENTRY;

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  // eslint-disable-line global-require
  app.quit();
}

const connectToServer = (window: BrowserWindow, server: FrontEndHost) => {
  discovery.stop();
  connectedServer = server;

  const url = getDebugHost() || frontEndHostUrl(server);

  window.loadURL(url);
};

const start = (): void => {
  // Create the browser window.
  const window = new BrowserWindow({
    height: 768,
    width: 1024,
    webPreferences: {
      preload: MAIN_WINDOW_PRELOAD_WEBPACK_ENTRY,
    },
  });

  // and load discovery (with autoconnect=true by default)
  window.loadURL(START_URL);

  ipcMain.on(IPC_MESSAGES.START_SERVER_DISCOVERY, () => {
    discovery.stop();
    discoveredServers = [];
    discovery.start();
  });

  ipcMain.on(IPC_MESSAGES.GO_BACK_TO_DISCOVERY, () => {
    window.loadURL(`${START_URL}?autoconnect=false`);
  });

  ipcMain.on(IPC_MESSAGES.CONNECT_TO_SERVER, (_event, server: FrontEndHost) =>
    connectToServer(window, server)
  );

  ipcMain.handle(IPC_MESSAGES.CONNECTED_SERVER, async () => connectedServer);
  ipcMain.handle(IPC_MESSAGES.START_BARCODE_SCAN, () =>
    barcodeScanner.start(window)
  );
  ipcMain.handle(IPC_MESSAGES.STOP_BARCODE_SCAN, () => barcodeScanner.stop());

  ipcMain.handle(IPC_MESSAGES.DISCOVERED_SERVERS, async () => {
    const servers = discoveredServers;
    discoveredServers = [];
    return { servers };
  });

  // not currently implemented in the desktop implementation
  ipcMain.on(IPC_MESSAGES.READ_LOG, () => 'Not implemented');

  discovery.on('serviceUp', function ({ type, port, addresses, txt }) {
    if (type?.name !== SERVICE_TYPE) return;
    if (typeof txt != 'object') return;

    const protocol = txt[PROTOCOL_KEY];
    const clientVersion = txt[CLIENT_VERSION_KEY];
    const hardwareId = txt[HARDWARE_ID_KEY];

    if (!isProtocol(protocol)) return;
    if (!(typeof clientVersion === 'string')) return;
    if (!(typeof hardwareId === 'string')) return;

    const ip = addresses.find(isV4Format);

    if (!ip) return;

    discoveredServers.push({
      port,
      protocol,
      ip,
      clientVersion: clientVersion || '',
      isLocal: ip === getIpAddress() || ip === '127.0.0.1',
      hardwareId,
    });
  });

  window.webContents.on(
    'did-fail-load',
    (_event, _errorCode, errorDescription, validatedURL) => {
      // not strictly necessary, done to prevent an infinite loop if the loadFile fails
      if (hasLoadingError) return;

      hasLoadingError = true;
      window.loadURL(
        `${START_URL}#/error?error=Failed to load URL ${validatedURL} with error: ${errorDescription}`
      );
    }
  );
};

app.on('ready', start);

app.on('window-all-closed', () => {
  app.quit();
});

process.on('uncaughtException', error => {
  // See comment below
  if (error.message.includes('[this.constructor.name] is not a constructor')) {
    return;
  }

  // TODO bugsnag ?
  dialog.showErrorBox('Error', error.stack || error.message);

  // The following error sometime occurs, it's dnssd related, it doesn't stop or break discovery, electron catching it and displays in error message, it's ignored by above if condition

  /* Uncaught Exception:
      TypeError: e[this.constructor.name] is not a constructor
      at t.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:77453)
      at ..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:49749
      at Array.reduce (<anonymous>)
      at t.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:49606)
      at t.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:49025)
      at ..open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:104855
      at Array.forEach (<anonymous>)
      at t.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:104807)
      at t.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:104661)
      at t.enter (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:8:99043)
 */
});

// App data store
type StoreType = {
  [key: string]: string | null;
};
const store = new ElectronStore<StoreType>();

app.addListener(
  'certificate-error',
  async (event, _webContents, url, error, certificate, callback) => {
    // We are only handling self signed certificate errors
    if (
      error != 'net::ERR_CERT_INVALID' &&
      error != 'net::ERR_CERT_AUTHORITY_INVALID'
    ) {
      return callback(false);
    }

    // Ignore SSL checks in debug mode
    if (getDebugHost()) {
      event.preventDefault();
      return callback(true);
    }

    // Default behaviour if not connected to a server or if url is not connectedServer

    if (!connectedServer) return callback(false);

    if (!url.startsWith(frontEndHostUrl(connectedServer))) {
      return callback(false);
    }

    // Match SSL fingerprint for server stored in app data

    // Match by hardware id and port
    const identifier = `${connectedServer.hardwareId}-${connectedServer.port}`;
    let storedFingerprint = store.get(identifier, null);

    // If fingerprint does not exists for server add it
    if (!storedFingerprint) {
      storedFingerprint = certificate.fingerprint;
      store.set(identifier, storedFingerprint);
      // If fingerprint does not match
    } else if (storedFingerprint != certificate.fingerprint) {
      // Display error message and go back to discovery
      const returnValue = await dialog.showMessageBox({
        type: 'warning',
        buttons: ['No', 'Yes'],
        title: 'SSL Error',
        message:
          'The security certificate on the server has changed!\r\n\r\nThis can happen when the server is reinstalled, so may be normal, but please check with your IT department if you are unsure.\r\n\r\nWould you like to accept the new certificate? ',
      });

      if (returnValue.response === 0) {
        ipcMain.emit(IPC_MESSAGES.GO_BACK_TO_DISCOVERY);
        return callback(false);
      }

      // Update stored fingerprint
      storedFingerprint = certificate.fingerprint;
      store.set(identifier, storedFingerprint);

      return callback(false);
    }

    // storedFingerprint did not exist or it matched certificate fingerprint
    event.preventDefault();
    return callback(true);
  }
);
