# Grammar & Rules — affiliate-config-parser

Файл `grammar.md` документує **розширену граматику** DSL-файлів, що парсяться цим проєктом, дає приклади, пояснює семантику кожного правила та вказує, як тестувати й обробляти помилки.

> Формат: простий текстовий DSL, ключі у верхньому регістрі, значення — рядки, числа або списки. Нові правила: `CAP`, `VERTICAL`. Також описано додаткові можливості (список, масив, map-подібні пари, умови).

---

## Загальний формат файлу
Файл складається з набору рядкових правил. Порядок правил не суворо фіксований, але деякі поля є **required** (обов'язкові): `OFFER`, `PAYOUT`, `CR`. Інші — опціональні.

Кожний рядок — окрема інструкція у форматі:

KEY: value

або (для блоків/масивів) — декілька рядків, наприклад:

KEY: a, b, c


---

## EBNF граматика (розширена)

```ebnf
Config        := { RuleLine }

RuleLine      := OfferRule
               | GeoRule
               | TrafficRule
               | PayoutRule
               | CRRule
               | CapRule
               | VerticalRule
               | MetaRule
               | Comment

OfferRule     := "OFFER:" WS? QuotedString
GeoRule       := "GEO:" WS? IdentList
TrafficRule   := "TRAFFIC:" WS? IdentList
PayoutRule    := "PAYOUT:" WS? Number WS? "USD"
CRRule        := "CR:" WS? Number "%" 
CapRule       := "CAP:" WS? Integer
VerticalRule  := "VERTICAL:" WS? Identifier
MetaRule      := Identifier ":" WS? Value

IdentList     := Ident { "," WS? Ident }
Ident         := Identifier | QuotedString

QuotedString  := '"' { ANY_EXCEPT_QUOTE } '"'
Identifier    := letter { letter | digit | '_' | '-' }
Number        := ["+"|"-"] digit { digit } [ "." digit { digit } ]
Integer       := digit { digit }
Value         := QuotedString | Number | IdentList | "[" Value { "," Value } "]" | "{" Pair { "," Pair } "}"
Pair          := Identifier ":" Value

Comment       := "#" { any }  ; whole-line comment
WS            := { " " | "\t" }

Правила (документація для кожного правила)
Кожне правило має окремий пояснювальний блок — це потрібно для docs.rs / оцінювання.

RULE: OFFER

Синтаксис: OFFER: "Offer Name"

Пояснення: Обов’язкове поле. Назва оффера. Якщо назва містить пробіли — використовуй лапки "...". Значення зберігається в OfferConfig.name.

Помилки: EmptyValue (якщо після OFFER: порожньо), DuplicateField (якщо вказано більше одного разу).

Приклад:

OFFER: "Crypto Pro Max"
RULE: GEO

Синтаксис: GEO: US, CA, UK

Пояснення: Список GEO кодів розділений комами. Кожний елемент — ідентифікатор (без лапок) або рядок у лапках. Мапується в OfferConfig.geo (Vec<String>). Підтримуються короткі коди країн (ISO-2).

Помилки: EmptyValue (якщо порожній список).

Приклад:

GEO: US, CA, FI

RULE: TRAFFIC

Синтаксис: TRAFFIC: Facebook, TikTok

Пояснення: Список джерел трафіку. Мапується в OfferConfig.traffic (Vec<String>).

Примітка: Значення можуть бути без лапок або в лапках, якщо містять пробіли.

Помилки: EmptyValue.

Приклад:

TRAFFIC: "Google UAC", Facebook

RULE: PAYOUT

Синтаксис: PAYOUT: 42.5 USD

Пояснення: Обов’язкове поле. Діапазон або десяткове число у валюті USD. Поточний парсер очікує число + USD. Зберігається як f32 у OfferConfig.payout.

Помилки: InvalidFormat (якщо відсутнє USD), InvalidNumber (якщо не число), DuplicateField.

Приклад:

PAYOUT: 180 USD
PAYOUT: 42.5 USD

RULE: CR

Синтаксис: CR: 1.25%

Пояснення: Обов’язкове поле. Конверсія у відсотках (з знаком %). Значення зберігається як f32 у OfferConfig.cr.

Помилки: InvalidFormat, InvalidNumber.

Приклад:

CR: 0.95%

RULE: CAP (нове)

Синтаксис: CAP: 100

Пояснення: Опціональне поле. Денний ліміт (ціле число). Мапується в OfferConfig.cap: Option<u32>.

Помилки: InvalidNumber, DuplicateField.

Приклад:

CAP: 200

RULE: VERTICAL (нове)

Синтаксис: VERTICAL: Crypto або VERTICAL: "Nutra - weight loss"

Пояснення: Опціональне поле, характеризує вертикаль продукту (Crypto, Nutra, Finance тощо). Мапується в OfferConfig.vertical: Option<String>.

Помилки: EmptyValue, DuplicateField.

Приклад:

VERTICAL: Crypto

RULE: MetaRule (універсальні key: value)

Синтаксис: KEY: value де KEY — будь-який ідентифікатор (UPPERCASE рекомендується).

Пояснення: Дозволяє розширювати DSL без зміни парсера (можна зберігати додаткові поля в map або ігнорувати за замовчуванням). Для цього проєкту парсер кидає UnknownRule для невідомих ключів — але в майбутньому можна додати пасивну фільтрацію.

COMMENTS

Формат: Рядок, що починається з # — ігнорується парсером.

Приклад:

# This is a comment line

Додаткові конструкції (пояснення для майбутнього розвитку)

Масиви: LIST: [a, b, c] — можна парсити як IdentList у квадратних дужках.

Мапи: {key: value, key2: value2} — корисно для складних параметрів.

Умови: IF: cap > 100 AND geo == US — складні умови не обовʼязкові в базовому проєкті, але граматика допускає їх у розширених версіях.

Приклади повного файлу

Простий валідний приклад:

OFFER: "Crypto Pro Max"
GEO: US, CA
TRAFFIC: Facebook, TikTok
PAYOUT: 42.5 USD
CR: 1.25%
CAP: 200
VERTICAL: Crypto


Приклад з коментарями і лапками:

# Offer for Nordics
OFFER: "Nordic VSL Offer"
GEO: NO, SE, FI
TRAFFIC: "Facebook Ads", "Native"
PAYOUT: 180 USD
CR: 0.95%
# optional fields omitted

Мапування граматики на парсер (коротко)

OFFER → OfferConfig.name (String)

GEO → OfferConfig.geo (Vec<String>)

TRAFFIC → OfferConfig.traffic (Vec<String>)

PAYOUT → OfferConfig.payout (f32)

CR → OfferConfig.cr (f32)

CAP → OfferConfig.cap (Option<u32>)

VERTICAL → OfferConfig.vertical (Option<String>)

Помилки (опис для тестів і docs.rs)

Io — помилка вводу/виводу при читанні файлу.

InvalidFormat — рядок не відповідає очікуваному формату (наприклад, відсутнє USD, відсутній %).

MissingField — відсутнє обовʼязкове поле (OFFER, PAYOUT, CR).

DuplicateField — поле визначене декілька разів (наприклад два рядки PAYOUT:).

UnknownRule — невідомий ключ (повинен бути використаний як помилка, поки ми не додали пасивну обробку).

InvalidNumber — не вдалось розпарсити число.

EmptyValue — після ключа немає значення.

Рекомендації для тест-кейсів

Positive tests (повинні проходити):

повний валідний файл з усіма полями

файл без опціональних полів (CAP, VERTICAL)

списки з пробілами та без (GEO:US,CA, GEO: US, CA)

quote handling (OFFER: "My Offer")

Negative tests (повинні падати):

відсутній OFFER

PAYOUT: 42 (без USD) → InvalidFormat

CR: 1.2 (без %) → InvalidFormat

CAP: not_a_number → InvalidNumber

дублювання полів → DuplicateField

невідомий ключ SOMETHING: x → UnknownRule (якщо парсер наразі кидає помилку)

Edge cases:

порожні рядки/коментарі між правилами

значення в лапках, що містять коми: TRAFFIC: "Google, Search", Facebook

Примітки для реалізації парсера
Парсер має працювати по рядку: trim(), пропуск порожніх рядків і коментарів.
Розпізнавання правил — starts_with("KEY:") або лексичний підхід.
При парсингу списків використовуйте split(',') та .trim() для кожного елементу.
Для числа з USD відсікати суфікс USD і парсити дробове число.
Для CR відсікати % і парсити число.
Для CAP парсити як u32.
Порядок правил не обовʼязковий; перевіряти обовʼязкові поля наприкінці.
Як додати нове правило (короткий how-to для dev)
Додати поле до OfferConfig (тип та опціональність).
Додати обробку ключа в parser.rs (перевірка duplicate / empty).
Додати unit test у tests/parser_tests.rs.
Додати опис правила в цьому файлі (grammar.md).

Версія граматики
v1.1 — базовий DSL + CAP, VERTICAL + покращені помилки (ця версія).

Заключне зауваження
Цей документ — офіційна технічна специфіка для парсера. Він потрібен для:
написання коректної реалізації парсера,
створення повних unit-тестів,
написання документації (docs.rs),
