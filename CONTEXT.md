# Prepper App

An iPhone app for managing a home prepper store: register what we buy, get reminded before it expires so we can use it and replace it. Used in normal times, before any emergency, not during one, so internet is assumed available. The app talks to our own backend server, which holds the inventory.

## Language

**Household**:
The group that shares one store. Owns all Units, Products, the Re-buy list, the Lead time and the calendar feed.

**Member**:
A person in the Household (two to begin with: each with their own phone and login). Members are created by hand on the server; there is no sign-up or invite flow. Every state change on a Unit is attributed to the Member who made it.
_Avoid_: user, account

**Unit**:
One physical can/package/bottle in the store. Has its own QR label, its own expiry date, and its own lifecycle. The atom of the system.
_Avoid_: item, batch, stock entry

**Product**:
The kind of thing a Unit is an instance of (e.g. "Chickpeas 400g"). Used to group Units for overview and re-buy. May carry a barcode (EAN): scanning a can's own barcode finds the Product in our own catalog, else in Open Food Facts, else we create it by hand. A Product never supplies an expiry date; that is always entered manually per purchase.
_Avoid_: SKU, type

**Label**:
The printed sticker attached to exactly one Unit: a QR code (a short URL to the Unit page) plus the Product name and expiry date in plain text, so the date is readable without scanning.
_Avoid_: tag, sticker

**Lifecycle**:
A Unit is **In store**, then ends as **Consumed** (used up) or **Discarded** (thrown away, expired/spoiled). Ended Units are kept as history, never deleted. No partial quantities or "opened" state in v1.

**Expired**:
An In-store Unit whose expiry date has passed without anyone Consuming or Discarding it. Shown red in the Store and listed in the Digest. Not a Lifecycle end state.

**Re-buy list**:
A list of Products we want to buy again. Consuming a Unit offers to add its Product to the list.
_Avoid_: shopping list

**Print agent**:
A small service on a Raspberry Pi at home, attached to a label printer (model not yet chosen; thermal receipt printers ruled out over BPA/BPS in the paper). It pulls pending print jobs from the backend (outbound connection only) and prints one Label per Unit. The app never talks to the printer directly.

**Scan**:
Using the app's built-in camera scanner. It reads either a Label (to act on that Unit, already authenticated) or a product's own barcode (to find or create a Product when registering a purchase). The stock iPhone camera opening a Label's URL is only the fallback path, via the Unit page.

**Unit page**:
The web page served by the backend at a Label's URL. Shows the Unit and offers Consume, Discard and Edit. Works from the stock iPhone camera with no app installed; the app is not required to act on a Label.

**Admin page**:
Backend-served web page for checking system status (Print agent online, pending print jobs, feed health, Units by state).

**Lead time**:
How many days before a Unit's expiry date we want to be reminded. One global value in v1 (no per-category).

**Reminder**:
Two all-day Calendar events per In-store Unit: "Use or replace: <Product> (expires <date>)" on expiry date minus Lead time, and "Expires today" on the expiry date. Each links to the Unit page. Delivered as a subscribed `.ics` feed published by the server, so the phone's Calendar app does the alerting. A Unit drops out of the feed when Consumed or Discarded. The primary alert mechanism, because it survives the app itself failing. (Assumption to verify: iOS honours alarms embedded in subscribed feeds.)

**Digest**:
Weekly local notifications, scheduled by the app about a year ahead and recomputed on every app open. Each is a dated snapshot ("As of 15 Nov: 3 Units expire by 20 Jan. Open the app for the current list."), so staleness is visible. The last queued one says notifications have run out. Secondary to Reminders.

**Register purchase**:
Adding one or more Units of a Product at once (same expiry) and printing a Label for each.
