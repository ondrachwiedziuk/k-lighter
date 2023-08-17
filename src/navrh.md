Automat
===
Implementace
---
### Automat
Čteme pole znaků
i - ukazatel na pole
buffer - zásobník, kam budeme ukládat znaky
state - stav automatu
color - flag značící barvu textu v bufferu
Jedna iterace - přečte znak. Podle přechodové funkce přejde do jiného stavu automatu. Pokud se se stavem pojí funkce, provede ji. Posune ukazatel o 1.

### Vnější tělo
Soubor čteme po řádcích. Nejprve načte slovo. Na slově spustí automat. Poté vezme slovo a zapíše do html souboru.

Funkce stavů
---
 - uložit do bufferu
 - vypráznit buffer (přiřadit barvu a zapsat do výstupu)
 - vrátit ukazatel o 1 zpět

## Stavy
### Se dopíše

## Barevná schémata
### numbers
### vars
### verbs
### adverbs
### space
### reserved
### pars_[1-8]
### comments