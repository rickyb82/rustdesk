# RustDesk per i clienti Sirius

Questo ramo (`claude/client-server-sirius`) è una **versione modificata di RustDesk 1.5.0**
(tag `1.5.0`, commit `fada664`), distribuita secondo la licenza AGPL-3.0 come l'originale.
Il nome e l'aspetto restano quelli di RustDesk; cambia solo il server.

## Cosa cambia rispetto a RustDesk 1.5.0 (14 ottobre 2026)

- **Server e chiave incorporati e bloccati.** Server ID e relay `emily.siriuslabs.it`
  (porte 21116 e 21117), chiave pubblica del server Sirius. Stanno nelle impostazioni "override"
  (`libs/base/src/config/builtin_server.rs`), che vincono su ogni valore salvato e che il
  programma rifiuta di cambiare: né la finestra ID/Relay (nascosta), né "Importa
  configurazione server", né `--config`, `--option` o `--import-config` li modificano.
- **Nessun server pubblico di RustDesk.** Anche i valori di riserva compilati in `hbb_common`
  puntano a Sirius (`.github/sirius/hbb_common.patch`, applicata dalla build); un ID scritto
  come `<id>@<altro server>` viene rifiutato; il relay è sempre quello di Sirius, anche quando
  chi chiede la connessione ne indica un altro.
- **Niente trucco del nome dell'installer**: rinominare l'exe non cambia server.
- **Niente server API** (`register-device=N`): il server OSS non ce l'ha.
- **Niente aggiornamenti ufficiali**: niente controllo della versione su `api.rustdesk.com`,
  niente avviso di nuova versione, niente aggiornamento automatico (installerebbe RustDesk
  ufficiale e cancellerebbe il server Sirius). Gli aggiornamenti li distribuiamo noi.
- **Nota AGPL** nella pagina Informazioni, con il link a questo sorgente.
- **CI**: un solo workflow, `Sirius Windows x64`; i workflow pianificati di RustDesk
  (nightly, webpki-roots) qui non partono da soli.

## Costruire l'installer

1. Nella scheda **Actions** del repository abilita i workflow (una volta sola).
2. In *Settings → General → Default branch* scegli `claude/client-server-sirius`:
   GitHub mostra i workflow manuali solo se stanno nel ramo predefinito.
3. *Actions → Sirius Windows x64 → Run workflow*. La prima volta dura circa
   1–2 ore (vcpkg), poi molto meno grazie alla cache.
4. Il risultato è l'artifact `rustdesk-1.5.0-sirius-windows-x86_64`: l'exe, l'MSI e
   `SHA256SUMS`. Con *release-tag* (per esempio `1.5.0-sirius`) viene pubblicata anche una
   prerelease.

L'installer **non è firmato**: Windows mostra l'avviso "editore sconosciuto". Il punto dove
aggiungere la firma è segnato nel workflow.

## Aggiornare a una nuova versione di RustDesk

Riportare le modifiche di questo ramo sul nuovo tag, rigenerare
`.github/sirius/hbb_common.patch` se `hbb_common/src/config.rs` è cambiato, riallineare le
versioni fissate nel workflow con `flutter-build.yml` del nuovo tag, ricompilare e
ridistribuire.

## Limiti noti

- **Stesso nome di RustDesk ufficiale.** Un installer ufficiale più nuovo, `winget upgrade`,
  Intune o altri gestori di aggiornamenti possono sostituirlo con RustDesk ufficiale (che non
  ha il server Sirius): sui PC dei clienti aggiornatelo solo voi, ed escludete RustDesk dagli
  aggiornamenti automatici di quegli strumenti.
- **Non accanto a un RustDesk ufficiale.** Prima di installarlo togliete RustDesk ufficiale; in
  versione portatile non va avviato su un PC dove gira il servizio ufficiale (condividono il
  canale interno e l'ID).
- **WebRTC e IPv6**, se qualcuno li attiva nelle impostazioni (di base sono spenti),
  interrogano server STUN pubblici di terzi (solo l'IP, nessun ID).
- **Solo Windows x64** per ora; macOS e Android non sono stati adattati.

Sorgente originale: https://github.com/rustdesk/rustdesk (AGPL-3.0).
