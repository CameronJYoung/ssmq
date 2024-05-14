install:
    ./scripts/install.bash

uninstall:
    sudo scripts/uninstall.bash

reinstall:
    ./scripts/uninstall.bash
    ./scripts/install.bash
