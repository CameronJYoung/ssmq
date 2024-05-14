install:
    ./scripts/install.bash

uninstall:
    ./scripts/uninstall.bash

reinstall:
    just uninstall
    just install
