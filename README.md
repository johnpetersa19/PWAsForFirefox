![Logo](https://rawcdn.githack.com/wiki/filips123/PWAsForFirefox/images/banner.svg)

Aplicativos Web Progressivos para Firefox
================================

[![Release](https://img.shields.io/github/v/release/filips123/PWAsForFirefox?sort=semver&style=flat-square)](https://github.com/filips123/PWAsForFirefox/releases/latest)
[![Users](https://img.shields.io/amo/users/pwas-for-firefox?style=flat-square)](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/)
[![Rating](https://img.shields.io/amo/rating/pwas-for-firefox?style=flat-square)](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/reviews/)
[![License](https://img.shields.io/github/license/filips123/PWAsForFirefox?style=flat-square)](https://github.com/filips123/PWAsForFirefox/blob/main/LICENSE)
[![Repositories](https://img.shields.io/repology/repositories/firefoxpwa?style=flat-square)](https://repology.org/project/firefoxpwa/versions)
[![Packagecloud.io DEB](https://img.shields.io/badge/deb-packagecloud.io-844fec.svg?style=flat-square)](https://packagecloud.io/filips/FirefoxPWA)
[![Packagecloud.io RPM](https://img.shields.io/badge/rpm-packagecloud.io-844fec.svg?style=flat-square)](https://packagecloud.io/filips/FirefoxPWA)

Uma ferramenta para instalar, gerenciar e usar Progressive Web Apps (PWAs) no Mozilla Firefox.

## Descrição

[Progressive Web Apps (PWAs)](https://developer.mozilla.org/docs/Web/Progressive_web_apps) são aplicativos web que utilizam APIs e recursos da web junto com uma estratégia de aprimoramento progressivo para proporcionar uma experiência de usuário semelhante a aplicativos nativos em aplicações web multiplataforma. Embora o Firefox suporte muitas das APIs de Progressive Web Apps, ele não suporta a funcionalidade de instalá-los como um aplicativo independente do sistema com uma experiência semelhante a um aplicativo.

Este projeto cria um ambiente Firefox modificado personalizado para permitir que websites sejam instalados como aplicativos independentes e fornece uma ferramenta de console e extensão do navegador para instalar, gerenciar e utilizá-los.

## Uso

**RESUMO**: Instale [a extensão do navegador](https://addons.mozilla.org/firefox/addon/pwas-for-firefox/) e siga as instruções de instalação no navegador. Você pode ler [o site da documentação](https://pwasforfirefox.filips.si/) para instruções de uso e outros recursos úteis.

Para mais detalhes e documentação técnica sobre configuração, uso e desenvolvimento do projeto, veja os READMEs das partes nativa e extensão:

* [Nativo](native/README.md)
* [Extensão](extension/README.md)

## Recursos

### Recursos Disponíveis

* Ferramenta de linha de comando para instalar, gerenciar e executar Progressive Web Apps no Firefox.
* Extensão para configurar programas nativos e instalar, gerenciar e executar PWAs e seus perfis diretamente do navegador Firefox principal.
* Instalação isolada do Firefox e perfil(s) que armazenam os PWAs.
* PWAs instalados têm sua própria entrada no menu iniciar/aplicativos e ícone na barra de tarefas, e funcionam em sua própria janela.
* PWAs instalados não têm abas e barra de endereço para uma melhor sensação de aplicativo.
* Suporte para instalar todos os websites como Progressive Web Apps.
* Suporte para todas as extensões do Firefox e recursos integrados do Firefox.
* Suporte para instalação automática (iniciada pelo usuário) e correção da instalação e perfil(s).

### Recursos Planejados

* Suporte para mais recursos do manifesto de aplicativos web relacionados ao sistema (após serem padronizados).

### Recursos Não Planejados

* **Integração no código oficial do Firefox.** Este projeto atualmente modifica a interface do navegador em tempo de execução usando JS e CSS. Embora isso funcione, não é oficialmente suportado pela Mozilla e pode quebrar com atualizações do Firefox. Para contribuir com recursos de volta ao código oficial do Firefox, eles precisariam ser implementados adequadamente com a nova página chrome e serviços do navegador. Infelizmente, isso requer uma reescrita quase completa do projeto, e atualmente não tenho conhecimento e tempo suficientes para fazer isso.

* **Usar o mesmo perfil de instalação para PWAs e navegação normal.** Isso poderia tornar a instalação/perfil principal do navegador instável se as coisas quebrarem. Também impediria a personalização do perfil PWA para funcionar melhor como um perfil PWA e a instalação de extensões personalizadas. Se você deseja sincronizar dados entre seu perfil principal e PWA, recomendo usar a Conta Firefox ou uma solução de sincronização de terceiros.

* **Executar PWAs instalados como pacotes Windows APPX/MSIX ou da Microsoft Store.** Eles sempre usarão o Edge baseado em Chromium instalado no Windows 10/11. Não tenho certeza se é possível substituir isso. Se não for muito difícil e não causar problemas, posso tentar isso no futuro.

* **Suporte para APIs específicas do Chromium (Sistema de arquivos, Bluetooth, NFC, USB...).** Isso exigiria bifurcar e modificar diretamente o código-fonte do Firefox. Além disso, não tenho certeza se dar aos websites os mesmos privilégios que aplicativos nativos é a melhor ideia...

### Limitações Atuais

Você pode consultar [nosso site de documentação](https://pwasforfirefox.filips.si/about/current-limitations/) para uma lista das limitações atuais.

## Apoiadores

### Patrocinadores

Agradecemos ao [packagecloud.io](https://packagecloud.io/) por patrocinar este projeto e nos fornecer hospedagem gratuita para nossos pacotes DEB e RPM!

  [<img src="https://assets-production.packagecloud.io/assets/packagecloud-logo-light-3c521566d5567fe0ce8435ef1f9485b0c3ad28a958af6f520d82ad3b232d2ff3.png" alt="Repositório NPM privado e Maven, RPM, DEB, PyPi e RubyGems · packagecloud" width="500">](https://packagecloud.io/)

&nbsp;

Agradecemos à [SignPath Foundation](https://signpath.org/) por nos fornecer um certificado de assinatura de código gratuito para pacotes Windows e ao [SignPath](https://about.signpath.io/) por fornecer a infraestrutura de assinatura de código!

  [<img src="https://signpath.org/assets/logo.svg" alt="Assinatura de Código Gratuita para software Open Source · SignPath" width="500">](https://signpath.org/)

&nbsp;

Agradecemos a todos os doadores por fornecerem suporte financeiro ao projeto!</br>
<sub>Por favor, verifique os [serviços de doação suportados](https://github.com/filips123/PWAsForFirefox?sponsor=1) se você deseja ajudar o projeto doando.</sub>

### Contribuidores

Agradecemos a [todos os contribuidores](https://github.com/filips123/PWAsForFirefox/graphs/contributors) deste projeto por fornecerem ajuda e desenvolverem recursos!

  [![Contribuidores](https://contrib.rocks/image?repo=filips123/PWAsForFirefox)](https://github.com/filips123/PWAsForFirefox/graphs/contributors)

### Outras Menções

Agradecemos a [todos os mantenedores de pacotes](https://repology.org/project/firefoxpwa/information) por garantirem que o projeto esteja atualizado! Agradecemos a [todos os tradutores](https://crowdin.com/project/firefoxpwa) por tornarem o projeto disponível em vários idiomas! Agradecemos a [todos que deram estrelas](https://github.com/filips123/PWAsForFirefox/stargazers) ao nosso repositório no GitHub. Finalmente, agradecemos à Mozilla e seus desenvolvedores por criarem o Firefox e tornarem possível modificar sua interface usando JavaScript!

## Versionamento

O projeto usa [SemVer](https://semver.org/) para versionamento. Para as versões disponíveis e o changelog, veja [os lançamentos](https://github.com/filips123/PWAsForFirefox/releases) neste repositório.

As partes nativa e extensão do projeto nas tags lançadas são compatíveis entre si de acordo com o SemVer. As partes nativa e extensão em commits não marcados podem não ser compatíveis entre si, pois são versões de desenvolvimento que podem não ser atualizadas ao mesmo tempo.

O projeto visa compatibilidade com a última versão estável do Firefox. Pode não ser compatível com as outras.

## Licença

O projeto está licenciado sob a Mozilla Public License 2.0. Ao usar, redistribuir ou modificá-lo, você deve concordar com a licença e as cláusulas adicionais fornecidas abaixo. Veja [o arquivo LICENSE](LICENSE) para o texto completo da licença.

O projeto usa ativos e código de terceiros adicionais:

* O logo do projeto é baseado no [ícone "Fox SVG Vector"](https://www.svgrepo.com/svg/40267/fox) e [o logo PWA introduzido pela comunidade](https://github.com/webmaxru/progressive-web-apps-logo), ambos dedicados ao domínio público usando [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

* As modificações da interface do navegador foram inspiradas e parcialmente derivadas do repositório [`xiaoxiaoflood/firefox-scripts`](https://github.com/xiaoxiaoflood/firefox-scripts) no GitHub, licenciado sob a Mozilla Public License 2.0. Informações detalhadas podem ser encontradas nos respectivos arquivos.

* As modificações da interface do navegador usam parcialmente código derivado do repositório [`black7375/Firefox-UI-Fix`](https://github.com/black7375/Firefox-UI-Fix) no GitHub, licenciado sob a Mozilla Public License 2.0. Informações detalhadas podem ser encontradas nos respectivos arquivos.

* As modificações da interface do navegador usam parcialmente código e ícones derivados do [código-fonte original do Firefox](https://github.com/mozilla/gecko-dev), licenciado sob a Mozilla Public License 2.0. Informações detalhadas podem ser encontradas nos respectivos arquivos.

* Os programas nativos contêm [a fonte Metropolis Semi Bold](https://fontsarena.com/metropolis-by-chris-simpson/) por Chris Simpson, liberada para o domínio público usando [Unlicense](https://unlicense.org/).

* O instalador Windows contém [Bootstrap Icons](https://icons.getbootstrap.com/), licenciado sob [a Licença MIT](https://opensource.org/licenses/MIT). Informações detalhadas sobre a licença podem ser encontradas no [arquivo de configuração WiX](native/packages/wix/main.wxs).

Software adicional de código aberto será baixado e instalado em tempo de execução quando iniciado pelo usuário:

* Instalar o ambiente no Windows instalará o [7-Zip](https://7-zip.org/) se ele ainda não estiver instalado. O projeto 7-Zip é feito por Igor Pavlov e [licenciado sob a licença GNU LGPL e outras](https://7-zip.org/license.txt). Este projeto não é afiliado ao projeto 7-Zip ou seus desenvolvedores de nenhuma forma.

* Instalar o ambiente em qualquer sistema baixará o [Mozilla Firefox](https://www.mozilla.org/firefox/) não modificado e o modificará localmente. Ao usar este projeto, você também concorda com [o Aviso de Privacidade do Firefox](https://www.mozilla.org/privacy/firefox/). O Firefox é licenciado sob a Mozilla Public License 2.0. Firefox e o logo do Firefox são marcas registradas da Mozilla Foundation nos EUA e outros países. Este projeto não é afiliado à Mozilla Foundation de nenhuma forma.
