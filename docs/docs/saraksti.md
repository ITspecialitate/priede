---
sidebar_position: 5
---

# Saraksti

Reizēm nepieciešams uzglabāt vairākas vērtības, kas ir ar vienādu nozīmi.

```priede
tk auglis1 : "ābols"
tk auglis2 : "bumbieris"
tk auglis3 : "banāns"
tk auglis4 : "apelsīns"
```

Šis piemērs nebūtu vēlams, jo, pirmkārt, jāraksta daudz teksta, otrkārt, nevar vienkārši pievienot vai noņemt vērtības.

Tā vietā var izmantot sarakstus

## Sarakstu definēšana

```priede
saraksts augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]
```

Saraksta definīcijai izmanto vārdu `saraksts`, kam seko saraksta nosaukums. Tad, kvadrātiekavās saraksta sākotnējās vērtības atdalītas ar semikoliem.

## Datu nolasīšana no saraksta

```priede
saraksts augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]

izvade(augļi[0])
```

Lai nolasītu saraksta konkrētu pozīciju izmanto saraksta nosaukumu, kuram seko kvadrātiekavas ar šī elementa pozīciju sarakstā, **sākot skaitīt no nulles**.

## Visu elementu izdruka

```priede
saraksts augļi : ["ābols";"bumbieris";"banāns";"apelsīns"]

sk skaitītajs : 0

kamēr skaitītajs <= 3 { //aizvieot 3 ar saraksta garumu
    izvade(augļi[skaitītajs])
    skaitītajs++
}
```
