@ECHO ##### Building omsupply for the postgres #####
SET installerWorkspace=C:\Program Files (x86)\Jenkins\jobs\omSupplyMain - installers\workspace\omSupply
cd "..\..\server" && cargo build --release --features postgres && cd .. && @ECHO ##### Copying build artifacts to the installer folder ##### && copy "server\target\release\remote_server.exe" "%installerWorkspace%\omSupply Web Server\omSupply-postgres.exe" && copy "server\target\release\remote_server.exe" "%installerWorkspace%\omSupply Desktop Server\server\omSupply-postgres.exe"
