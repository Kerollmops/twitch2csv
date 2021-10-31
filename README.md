# twitch2csv

A tool to stream the chats of Twitch channels as a CSV.

## Installation

You can use cargo to install this tool:

```bash
cargo install -f twitch2csv
```

## Usage

You can follow channels by giving listing the channel names:

```bash
twitch2csv joueur_du_grenier mistermv
```

This outputs a CSV like this:

```csv
server-timestamp,channel-login,sender-id,sender-login,sender-name,message-id,message-text
1635692267,joueur_du_grenier,46241346,le_lowis,Le_Lowis,TPK perma ban,b153d255-abe3-4669-861c-a67d6dac364a
1635692267,joueur_du_grenier,100658423,nathan43005,Nathan43005,Qui est dans l'équipe de bob ? j'arrive pas à reconnaitre,708b7b79-0307-4d16-81f7-7a11152eb175
1635692267,joueur_du_grenier,204918847,joojoow,joojoow,mais il y aura assez de questions ? LUL,b6d07625-ae08-4880-9495-f0dd09c589e4
1635692268,joueur_du_grenier,48052347,shelburn,shelburn,ONE PIECE,3a23281b-87bd-4fe1-a5a8-64463bc2347e
1635692268,joueur_du_grenier,504187358,mayellebalkanywest,MayelleBalkanywest,Ultia la stratège yeees,14b0ff56-e558-4930-a1cd-954dbc12302c
1635692268,joueur_du_grenier,45803334,haisenbear,HaisenBear,non mais laissez le jouer,60cbbd0c-f2e5-47b6-a480-417aea833b10
1635692268,joueur_du_grenier,138120818,sprite11100,sprite11100,One piece nouveau chap dans 3min,c9073759-4c49-49f4-ba26-30acd93250d9
1635692269,joueur_du_grenier,205431180,golgoth_75,golgoth_75,<3<3<3,e643384b-6bcc-4441-949e-c065c00a4bfe
1635692269,joueur_du_grenier,30655211,jonvon_,JonVon_,any TWITCH PRIMERS,3fe5c39b-dd28-476b-a75f-34781f872f7b
1635692270,joueur_du_grenier,39673409,harma_,Harma_,one piece ils prendront jamais ces vieux,e01c5f7b-9725-449d-8eb7-a7aa0cf0ed3f
1635692270,joueur_du_grenier,267119246,espernobu,espernobu,LUL,660885d1-4dda-4642-8767-268e39305e3b
1635692270,joueur_du_grenier,549290335,polpoxe,polpoxe,CANAL J oooohhhh ouiiii,3af25b8d-0165-46a2-8f80-ebda27c83d6c
1635692270,joueur_du_grenier,541571810,xxmxdnxx,xxmxdnxx,Ce blanc,3fcd55ac-be04-4c3b-8631-1be3f122f29a
```
