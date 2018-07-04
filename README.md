# petrobot
Der Petro Bot hilft den Bewohnern der Petronellastraße die Hürden des alltäglichen Lebens zu meistern.

## command list
|command|description|
|---|---|
|`/entendienst`|Gibt zurück, wer sich an dem Tag um das Einsperren der Enten kümmert.|
|`/entenpapa`|Setzt den Nutzer als Entenpapa. Verpflichtet ihn dazu, sich an dem Tag um das Einsperren der Enten zu kümmern.|
|`/entenmama`|Dadurch wirst du zur Enten Mama und musst dich heute um das Einsperren der Enten kümmern.|
|`/entenpunkte`|Gibt zurück wie viele Entenpunkte die WG-Teilnehmer gesammelt haben.|
|`/todo`|Fügt den nachfolgenden Eintrag in die Todo-Liste ein.|
|`/todo_loeschen`|Löscht das i.te Element aus der Todo-Liste.|
|`/todoliste`|Gibt die Todo-Liste aus.|
|`/einkaufsliste`|Gibt die Einkaufsliste aus.|
|`/einkauf`|Fügt den nachfolgenden Text zur Einkaufsliste hinzu.|
|`/einkaufsliste_loeschen`|Löscht die aktuelle Einkaufsliste.|
|`/pool`|Gibt die aktuelle Pooltemperatur aus.|


## install
### sources
sudo apt-get update
sudo apt-get install git-core
sudo git clone https://github.com/ossel/petrobot.git
### rust
curl https://sh.rustup.rs -sSf | sh
export PATH="$HOME/.cargo/bin:$PATH"
### openssl
sudo apt-get install pkg-config libssl-dev
### set env vars
nano /etc/environment
TELEGRAM_BOT_TOKEN ="<token>"
TELEGRAM_BOT_CHAT_ID = "<id>"
