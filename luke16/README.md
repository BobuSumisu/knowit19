# Seilkryss
Av: Didrik Pemmer Aalen

Birte befinner seg på en seilbåt helt innerst i den lange julefjorden. Hun har et ønske om å komme seg ut, slik at hun kommer seg hjem til jul, men vindforholdene gjør det vanskelig. For å komme seg ut av fjorden må hun følge disse reglene:

Hun kan kun krysse nordøst (x+1, y+1) og sørøst (x+1, y-1).
For å krysse færrest mulige ganger snur hun når hun er 20 meter fra land (nord/sør).
Oppgave
Gitt følgende fjord, hvor mange ganger ender Birte opp med å krysse for å komme seg ut av fjorden når hun begynner å krysse mot nordøst?

## Eksempel
Her er følgende definert:

B = Birte
# = Land
Tomme ruter er vann
/ og \ viser ruten til Birte i svaret
Hver rute er 10m2

Gitt følgende fjord:

    #####################
    #    ###
    #
    #
    #
    #
    #
    #
    #B
    ###    ####     #   #
    #####################

Blir kryssingene slik:

    #####################
    #    ###
    #
    #           /\      /
    #    /\    /  \    /
    #   /  \  /    \  /
    #  /    \/      \/
    # /
    #B
    ###    ####     #   #
    #####################

Og Birte ender med 5 kryssinger.
