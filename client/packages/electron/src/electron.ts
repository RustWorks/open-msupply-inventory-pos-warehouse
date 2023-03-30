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

  start() {
    return new Promise((resolve, reject) => {
      if (!this.device) reject(new Error('No scanners found'));
      try {
        this.device?.read((err, data) => {
          if (err) reject(err);
          resolve(data);
        });
      } catch (e) {
        reject(e);
      }
    });
  }

  stop() {
    this.device?.close();
    this.device = this.findDevice();
  }
}

const discovery = new dnssd.Browser(dnssd.tcp(SERVICE_TYPE));
const barcodeScanner = new BarcodeScanner();

let connectedServer: FrontEndHost | null = null;
let discoveredServers: FrontEndHost[] = [];

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
  ipcMain.handle(IPC_MESSAGES.START_BARCODE_SCAN, () => barcodeScanner.start());
  ipcMain.on(IPC_MESSAGES.STOP_BARCODE_SCAN, () => barcodeScanner.stop());

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
};
app.on('ready', start);

app.on('window-all-closed', () => {
  app.quit();
});

process.on('uncaughtException', error => {
  // See comment below
  if (
    error.message.includes('t[this.constructor.name] is not a constructor') &&
    error.stack?.includes('v._addKnownAnswers')
  ) {
    return;
  }

  // TODO bugsnag ?
  dialog.showErrorBox('Error', error.stack || error.message);

  // The following error sometime occurs, it's dnssd related, it doesn't stop or break discovery, electron catching it and displays in error message, it's ignored by above if condition

  /* Uncaught Exception:
        TypeError: t[this.constructor.name] is not a constructor
        at e.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:48513)
        at ..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:20805
        at Array.reduce (<anonymous>)
        at e.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:20662)
        at e.value (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:20277)
        at v._addKnownAnswers (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:39980)
        at v._send (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:39523)
        at Timeout._onTimeout (..open mSupply-darwin-arm64/open mSupply.app/Contents/Resources/app/.webpack/main/index.js:2:82424)
        at listOnTimeout (node:internal/timers:559:17)
        at process.processTimers (node:internal/timers:502:7)
  */
});

// App data store
type StoreType = {
  [key: string]: string | null;
};
const store = new ElectronStore<StoreType>();

app.addListener(
  'certificate-error',
  (event, _webContents, url, error, certificate, callback) => {
    // We are only handling self signed certificate errors
    if (error != 'net::ERR_CERT_INVALID') {
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
      dialog.showErrorBox(
        'SSL Error',
        'Certificate fingerprint for server was changed'
      );
      ipcMain.emit(IPC_MESSAGES.GO_BACK_TO_DISCOVERY);

      return callback(false);
    }

    // storedFingerprint did not exist or it matched certificaite fingerprint
    event.preventDefault();
    return callback(true);
  }
);
