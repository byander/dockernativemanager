# 1.0.0 (2026-07-17)


### Bug Fixes

* adicionar @semantic-release/github e corrigir disparo do release build ([0fff974](https://github.com/byander/dockernativemanager/commit/0fff97494260a2eaabd1c6c1669b4af6f0a8667f))
* adicionar handles de redimensionamento programáticos para compatibilidade com Wayland ([53a9357](https://github.com/byander/dockernativemanager/commit/53a935713dc83bf94a341a158943d526535e81a4))
* adjust arch linux packaging command and quoting ([34ac21c](https://github.com/byander/dockernativemanager/commit/34ac21ce9ee2a2da316fcdbb4cdb16c076e76e42))
* ajusta lint para destravar o CI ([b70fb81](https://github.com/byander/dockernativemanager/commit/b70fb81bbf5dbd50627b797b8993acd756e663a1))
* ajuste no double click na header para expandir a janela ([3824ed9](https://github.com/byander/dockernativemanager/commit/3824ed9a26940a0362d0aaf10a10806d96ba316d))
* allow pacman to run without password in arch container ([c27021c](https://github.com/byander/dockernativemanager/commit/c27021c6026819b2f4bb9433ff11659381a82694))
* aprovar build scripts do @swc/core e esbuild no pnpm 11 ([823e8b0](https://github.com/byander/dockernativemanager/commit/823e8b0872139f8a9f1e2ede4d84199251ae5b89))
* atualização da versão para 1.5.3 ([3a8d29d](https://github.com/byander/dockernativemanager/commit/3a8d29d915f45b87e190d808bc238619ae33a08e))
* build do Tauri no mesmo workflow do semantic-release ([7eb1053](https://github.com/byander/dockernativemanager/commit/7eb1053ee2e28a15b64ee46a683e3657187470dc))
* bump version to 1.4.1 ([3af153c](https://github.com/byander/dockernativemanager/commit/3af153cd9a626dac130c539803d7b0be0cbb1bc0))
* **ci:** adjust icon path for Arch Linux build and upgrade to Node 24 due to CI warnings ([b957ec2](https://github.com/byander/dockernativemanager/commit/b957ec22cc72e317d9166badd956da23c32d1573))
* **ci:** falha do pnpm no CI ([1bc08b2](https://github.com/byander/dockernativemanager/commit/1bc08b2ebb024468d1d88de78112ba79fcec7675))
* **ci:** fix Arch Linux pkg.tar.zst generation and desktop entry install ([8aa3fc7](https://github.com/byander/dockernativemanager/commit/8aa3fc777ce0430cef02b7f8c94ce00f7b0c4176))
* **ci:** update version with CI improvments for Arch Linux and Tarball ([63870c9](https://github.com/byander/dockernativemanager/commit/63870c9a5c1a20cb78930f844b3f844dc9e611c3))
* estabiliza workflow de release para linux e arch ([b8c4ce7](https://github.com/byander/dockernativemanager/commit/b8c4ce7a7fc8a3308cd26b759088ede018a9eee6))
* mover onlyBuiltDependencies para pnpm-workspace.yaml ([2cac5b1](https://github.com/byander/dockernativemanager/commit/2cac5b1e0ec92db76d5b76a35228d26d5551f121))
* reimplementar drag e duplo clique da titlebar via pointerdown ([1c1311a](https://github.com/byander/dockernativemanager/commit/1c1311ab6d9a8cc55fc8c68cbdcd296e538c7ad7))
* remover version fixa do pnpm/action-setup para usar packageManager do package.json ([8ec9403](https://github.com/byander/dockernativemanager/commit/8ec9403215df8738292e4ab9e749c5857052fd9f))
* restore text selection and copy restrictions in production ([547203d](https://github.com/byander/dockernativemanager/commit/547203d498b4f9ba87f10cceb34a8430d0919fc6))
* set tauri bundle targets to all ([2932663](https://github.com/byander/dockernativemanager/commit/293266357343354a5bcd637c999486d186fc027b))
* update resizable wrapper for v4 ([b22aed6](https://github.com/byander/dockernativemanager/commit/b22aed6723273ee011130823060944b397da6050))


### Features

* add versioning script and automated release support ([501c2a6](https://github.com/byander/dockernativemanager/commit/501c2a61720e13f51b6bbf2c9c31214dee91531d))
* add window shadow and adjust layout for non-maximized state ([845403b](https://github.com/byander/dockernativemanager/commit/845403b91c14fda055d8ab7c75e2d674739ad337))
* adiciona controle do daemon e melhora reatividade do app ([b931932](https://github.com/byander/dockernativemanager/commit/b9319326369039cd9b36732c1465c87510ca306b))
* comprehensive UI/UX overhaul and functional enhancements ([959d93a](https://github.com/byander/dockernativemanager/commit/959d93aa3f67bcbf6459041273fde52ad2228cd7)), closes [hi#fidelity](https://github.com/hi/issues/fidelity)
* comprehensive update to resource management and UI modernization ([d92f0d1](https://github.com/byander/dockernativemanager/commit/d92f0d16b3a6a8a93a536709e3aa05eb43603e7e))
* implement advanced container management, themes, and dashboard UI improvements ([3200c55](https://github.com/byander/dockernativemanager/commit/3200c55ad7714b00932d77cea6afe8b53779f898))
* implement advanced container management, themes, dashboard UI improvements, and bump version to 1.1.0 ([d5c2494](https://github.com/byander/dockernativemanager/commit/d5c24942d77da4cb68cbb46e7a5321d6ed6987c4))
* implement full host stats (CPU, Mem, Disk, Net) for remote SSH contexts ([b75d453](https://github.com/byander/dockernativemanager/commit/b75d4539930c512e55e92b39ac4e8503c2aa6ff2))
* implement interactive terminal with shell/user selection; improve panel UI with animations and dynamic theming ([ca94620](https://github.com/byander/dockernativemanager/commit/ca946205469ec8dfe99bdd508587665fa07dddfd))
* implementar estatísticas completas do host (CPU, Mem, Disco, Rede) para contextos remotos via SSH e corrigir conflitos de túnel (v1.9.0) ([e34f26f](https://github.com/byander/dockernativemanager/commit/e34f26f1a7ff53207093df015feb789990db6886))
* implementar gerenciamento de arquivos em volumes (navegação, upload e exclusão) ([c689e22](https://github.com/byander/dockernativemanager/commit/c689e223019bcca7df0c6aa0c38c659755737fbf))
* implementar semantic-release com branches main (stable) e dev (beta) ([e6dc0fa](https://github.com/byander/dockernativemanager/commit/e6dc0fa4f623217a4d9852dc0114367104d5acca))
* improve logs viewer, persist stack compose files, and standardize wider sheet UI across all pages ([ae3ed52](https://github.com/byander/dockernativemanager/commit/ae3ed52aae488e0a46a58ff17f10cd7617a7b62f))
* melhoria no fluxo de deploy de stacks com upload de .env, responsividade e correção de bugs ([b708922](https://github.com/byander/dockernativemanager/commit/b708922cc683bbbcae5495cacb0b9984a7e7691e))
* melhorias no sistema de atualização, stacks swarm e informações de imagens ([b6fccd8](https://github.com/byander/dockernativemanager/commit/b6fccd8b8229a8a2525ddb51de720333cbc336e6))
* move container status to actions menu and customize table columns for containers and stacks ([1d6de8e](https://github.com/byander/dockernativemanager/commit/1d6de8ee71ba108b65a77c48cbb059909288450e))
* otimiza release, fixa arch linux e atualiza node.js no runner ([9528c70](https://github.com/byander/dockernativemanager/commit/9528c70d9f3777ab985ebe2d246ee0f1b8c09a70))
* replace terminal svg with dnm-icon, dynamic version from tauri, disable right-click/drag in prod ([ad758dc](https://github.com/byander/dockernativemanager/commit/ad758dcd80b017bdeb24c68fea8e404214699767))
* sistema de atualização, modal sobre e correção do status do daemon ([25872e0](https://github.com/byander/dockernativemanager/commit/25872e0910f9fddaa488cd27213c66f6e736221a))
* Suporte a conexão remota via SSH com gerenciamento de chaves ([1482d6b](https://github.com/byander/dockernativemanager/commit/1482d6b1a857096164a7d5c24e20c18282b47070))
* v1.6.0 - suporte inicial ao Docker Swarm e Configuração de Clusters (Contextos Docker) ([ed2edc7](https://github.com/byander/dockernativemanager/commit/ed2edc77fa23f911fca655a99a4e225e0b6569a4))

# 1.0.0-beta.1 (2026-07-17)


### Bug Fixes

* adicionar @semantic-release/github e corrigir disparo do release build ([0fff974](https://github.com/byander/dockernativemanager/commit/0fff97494260a2eaabd1c6c1669b4af6f0a8667f))
* adicionar handles de redimensionamento programáticos para compatibilidade com Wayland ([53a9357](https://github.com/byander/dockernativemanager/commit/53a935713dc83bf94a341a158943d526535e81a4))
* adjust arch linux packaging command and quoting ([34ac21c](https://github.com/byander/dockernativemanager/commit/34ac21ce9ee2a2da316fcdbb4cdb16c076e76e42))
* ajusta lint para destravar o CI ([b70fb81](https://github.com/byander/dockernativemanager/commit/b70fb81bbf5dbd50627b797b8993acd756e663a1))
* ajuste no double click na header para expandir a janela ([3824ed9](https://github.com/byander/dockernativemanager/commit/3824ed9a26940a0362d0aaf10a10806d96ba316d))
* allow pacman to run without password in arch container ([c27021c](https://github.com/byander/dockernativemanager/commit/c27021c6026819b2f4bb9433ff11659381a82694))
* aprovar build scripts do @swc/core e esbuild no pnpm 11 ([823e8b0](https://github.com/byander/dockernativemanager/commit/823e8b0872139f8a9f1e2ede4d84199251ae5b89))
* atualização da versão para 1.5.3 ([3a8d29d](https://github.com/byander/dockernativemanager/commit/3a8d29d915f45b87e190d808bc238619ae33a08e))
* build do Tauri no mesmo workflow do semantic-release ([7eb1053](https://github.com/byander/dockernativemanager/commit/7eb1053ee2e28a15b64ee46a683e3657187470dc))
* bump version to 1.4.1 ([3af153c](https://github.com/byander/dockernativemanager/commit/3af153cd9a626dac130c539803d7b0be0cbb1bc0))
* **ci:** adjust icon path for Arch Linux build and upgrade to Node 24 due to CI warnings ([b957ec2](https://github.com/byander/dockernativemanager/commit/b957ec22cc72e317d9166badd956da23c32d1573))
* **ci:** falha do pnpm no CI ([1bc08b2](https://github.com/byander/dockernativemanager/commit/1bc08b2ebb024468d1d88de78112ba79fcec7675))
* **ci:** fix Arch Linux pkg.tar.zst generation and desktop entry install ([8aa3fc7](https://github.com/byander/dockernativemanager/commit/8aa3fc777ce0430cef02b7f8c94ce00f7b0c4176))
* **ci:** update version with CI improvments for Arch Linux and Tarball ([63870c9](https://github.com/byander/dockernativemanager/commit/63870c9a5c1a20cb78930f844b3f844dc9e611c3))
* estabiliza workflow de release para linux e arch ([b8c4ce7](https://github.com/byander/dockernativemanager/commit/b8c4ce7a7fc8a3308cd26b759088ede018a9eee6))
* mover onlyBuiltDependencies para pnpm-workspace.yaml ([2cac5b1](https://github.com/byander/dockernativemanager/commit/2cac5b1e0ec92db76d5b76a35228d26d5551f121))
* reimplementar drag e duplo clique da titlebar via pointerdown ([1c1311a](https://github.com/byander/dockernativemanager/commit/1c1311ab6d9a8cc55fc8c68cbdcd296e538c7ad7))
* remover version fixa do pnpm/action-setup para usar packageManager do package.json ([8ec9403](https://github.com/byander/dockernativemanager/commit/8ec9403215df8738292e4ab9e749c5857052fd9f))
* restore text selection and copy restrictions in production ([547203d](https://github.com/byander/dockernativemanager/commit/547203d498b4f9ba87f10cceb34a8430d0919fc6))
* set tauri bundle targets to all ([2932663](https://github.com/byander/dockernativemanager/commit/293266357343354a5bcd637c999486d186fc027b))
* update resizable wrapper for v4 ([b22aed6](https://github.com/byander/dockernativemanager/commit/b22aed6723273ee011130823060944b397da6050))


### Features

* add versioning script and automated release support ([501c2a6](https://github.com/byander/dockernativemanager/commit/501c2a61720e13f51b6bbf2c9c31214dee91531d))
* add window shadow and adjust layout for non-maximized state ([845403b](https://github.com/byander/dockernativemanager/commit/845403b91c14fda055d8ab7c75e2d674739ad337))
* adiciona controle do daemon e melhora reatividade do app ([b931932](https://github.com/byander/dockernativemanager/commit/b9319326369039cd9b36732c1465c87510ca306b))
* comprehensive UI/UX overhaul and functional enhancements ([959d93a](https://github.com/byander/dockernativemanager/commit/959d93aa3f67bcbf6459041273fde52ad2228cd7)), closes [hi#fidelity](https://github.com/hi/issues/fidelity)
* comprehensive update to resource management and UI modernization ([d92f0d1](https://github.com/byander/dockernativemanager/commit/d92f0d16b3a6a8a93a536709e3aa05eb43603e7e))
* implement advanced container management, themes, and dashboard UI improvements ([3200c55](https://github.com/byander/dockernativemanager/commit/3200c55ad7714b00932d77cea6afe8b53779f898))
* implement advanced container management, themes, dashboard UI improvements, and bump version to 1.1.0 ([d5c2494](https://github.com/byander/dockernativemanager/commit/d5c24942d77da4cb68cbb46e7a5321d6ed6987c4))
* implement full host stats (CPU, Mem, Disk, Net) for remote SSH contexts ([b75d453](https://github.com/byander/dockernativemanager/commit/b75d4539930c512e55e92b39ac4e8503c2aa6ff2))
* implement interactive terminal with shell/user selection; improve panel UI with animations and dynamic theming ([ca94620](https://github.com/byander/dockernativemanager/commit/ca946205469ec8dfe99bdd508587665fa07dddfd))
* implementar estatísticas completas do host (CPU, Mem, Disco, Rede) para contextos remotos via SSH e corrigir conflitos de túnel (v1.9.0) ([e34f26f](https://github.com/byander/dockernativemanager/commit/e34f26f1a7ff53207093df015feb789990db6886))
* implementar gerenciamento de arquivos em volumes (navegação, upload e exclusão) ([c689e22](https://github.com/byander/dockernativemanager/commit/c689e223019bcca7df0c6aa0c38c659755737fbf))
* implementar semantic-release com branches main (stable) e dev (beta) ([e6dc0fa](https://github.com/byander/dockernativemanager/commit/e6dc0fa4f623217a4d9852dc0114367104d5acca))
* improve logs viewer, persist stack compose files, and standardize wider sheet UI across all pages ([ae3ed52](https://github.com/byander/dockernativemanager/commit/ae3ed52aae488e0a46a58ff17f10cd7617a7b62f))
* melhoria no fluxo de deploy de stacks com upload de .env, responsividade e correção de bugs ([b708922](https://github.com/byander/dockernativemanager/commit/b708922cc683bbbcae5495cacb0b9984a7e7691e))
* melhorias no sistema de atualização, stacks swarm e informações de imagens ([b6fccd8](https://github.com/byander/dockernativemanager/commit/b6fccd8b8229a8a2525ddb51de720333cbc336e6))
* move container status to actions menu and customize table columns for containers and stacks ([1d6de8e](https://github.com/byander/dockernativemanager/commit/1d6de8ee71ba108b65a77c48cbb059909288450e))
* otimiza release, fixa arch linux e atualiza node.js no runner ([9528c70](https://github.com/byander/dockernativemanager/commit/9528c70d9f3777ab985ebe2d246ee0f1b8c09a70))
* replace terminal svg with dnm-icon, dynamic version from tauri, disable right-click/drag in prod ([ad758dc](https://github.com/byander/dockernativemanager/commit/ad758dcd80b017bdeb24c68fea8e404214699767))
* sistema de atualização, modal sobre e correção do status do daemon ([25872e0](https://github.com/byander/dockernativemanager/commit/25872e0910f9fddaa488cd27213c66f6e736221a))
* Suporte a conexão remota via SSH com gerenciamento de chaves ([1482d6b](https://github.com/byander/dockernativemanager/commit/1482d6b1a857096164a7d5c24e20c18282b47070))
* v1.6.0 - suporte inicial ao Docker Swarm e Configuração de Clusters (Contextos Docker) ([ed2edc7](https://github.com/byander/dockernativemanager/commit/ed2edc77fa23f911fca655a99a4e225e0b6569a4))

## [1.13.1](https://github.com/byander/dockernativemanager/compare/v1.13.0...v1.13.1) (2026-07-17)


### Bug Fixes

* reimplementar drag e duplo clique da titlebar via pointerdown ([1c1311a](https://github.com/byander/dockernativemanager/commit/1c1311ab6d9a8cc55fc8c68cbdcd296e538c7ad7))

## [1.13.1-beta.1](https://github.com/byander/dockernativemanager/compare/v1.13.0...v1.13.1-beta.1) (2026-07-17)


### Bug Fixes

* reimplementar drag e duplo clique da titlebar via pointerdown ([1c1311a](https://github.com/byander/dockernativemanager/commit/1c1311ab6d9a8cc55fc8c68cbdcd296e538c7ad7))

# [1.13.0](https://github.com/byander/dockernativemanager/compare/v1.12.0...v1.13.0) (2026-07-17)


### Bug Fixes

* adicionar @semantic-release/github e corrigir disparo do release build ([0fff974](https://github.com/byander/dockernativemanager/commit/0fff97494260a2eaabd1c6c1669b4af6f0a8667f))
* adicionar handles de redimensionamento programáticos para compatibilidade com Wayland ([53a9357](https://github.com/byander/dockernativemanager/commit/53a935713dc83bf94a341a158943d526535e81a4))
* aprovar build scripts do @swc/core e esbuild no pnpm 11 ([823e8b0](https://github.com/byander/dockernativemanager/commit/823e8b0872139f8a9f1e2ede4d84199251ae5b89))
* build do Tauri no mesmo workflow do semantic-release ([7eb1053](https://github.com/byander/dockernativemanager/commit/7eb1053ee2e28a15b64ee46a683e3657187470dc))
* mover onlyBuiltDependencies para pnpm-workspace.yaml ([2cac5b1](https://github.com/byander/dockernativemanager/commit/2cac5b1e0ec92db76d5b76a35228d26d5551f121))
* remover version fixa do pnpm/action-setup para usar packageManager do package.json ([8ec9403](https://github.com/byander/dockernativemanager/commit/8ec9403215df8738292e4ab9e749c5857052fd9f))


### Features

* implementar semantic-release com branches main (stable) e dev (beta) ([e6dc0fa](https://github.com/byander/dockernativemanager/commit/e6dc0fa4f623217a4d9852dc0114367104d5acca))

# [1.12.0-beta.3](https://github.com/byander/dockernativemanager/compare/v1.12.0-beta.2...v1.12.0-beta.3) (2026-07-17)


### Bug Fixes

* build do Tauri no mesmo workflow do semantic-release ([7eb1053](https://github.com/byander/dockernativemanager/commit/7eb1053ee2e28a15b64ee46a683e3657187470dc))

# [1.12.0-beta.2](https://github.com/byander/dockernativemanager/compare/v1.12.0-beta.1...v1.12.0-beta.2) (2026-07-17)


### Bug Fixes

* adicionar @semantic-release/github e corrigir disparo do release build ([0fff974](https://github.com/byander/dockernativemanager/commit/0fff97494260a2eaabd1c6c1669b4af6f0a8667f))

# [1.12.0-beta.1](https://github.com/byander/dockernativemanager/compare/v1.11.0...v1.12.0-beta.1) (2026-07-17)


### Bug Fixes

* adicionar handles de redimensionamento programáticos para compatibilidade com Wayland ([53a9357](https://github.com/byander/dockernativemanager/commit/53a935713dc83bf94a341a158943d526535e81a4))
* ajusta lint para destravar o CI ([b70fb81](https://github.com/byander/dockernativemanager/commit/b70fb81bbf5dbd50627b797b8993acd756e663a1))
* aprovar build scripts do @swc/core e esbuild no pnpm 11 ([823e8b0](https://github.com/byander/dockernativemanager/commit/823e8b0872139f8a9f1e2ede4d84199251ae5b89))
* mover onlyBuiltDependencies para pnpm-workspace.yaml ([2cac5b1](https://github.com/byander/dockernativemanager/commit/2cac5b1e0ec92db76d5b76a35228d26d5551f121))
* remover version fixa do pnpm/action-setup para usar packageManager do package.json ([8ec9403](https://github.com/byander/dockernativemanager/commit/8ec9403215df8738292e4ab9e749c5857052fd9f))
* update resizable wrapper for v4 ([b22aed6](https://github.com/byander/dockernativemanager/commit/b22aed6723273ee011130823060944b397da6050))


### Features

* implementar semantic-release com branches main (stable) e dev (beta) ([e6dc0fa](https://github.com/byander/dockernativemanager/commit/e6dc0fa4f623217a4d9852dc0114367104d5acca))
