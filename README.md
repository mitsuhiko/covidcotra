# covidcotra

This is an experimental rust library for managing the data exchange for a
covid-19 contract tracing system.

## Concepts

Conceptionally there are three components:

- [`Identity`](https://docs.rs/covidcotra/latest/covidcotra/struct.Identity.html) represents a single device.  However
  for extra safety a device can rotate IDs for as long as it also polls
  for some period of time on the older IDs.
- [`Authority`](https://docs.rs/covidcotra/latest/covidcotra/struct.Authority.html) represents a trusted authority
  (for instance the ministry of health)
- [`ContactLog`](https://docs.rs/covidcotra/latest/covidcotra/struct.ContactLog.html) is a minimal abstraction over a
  contact log.

## Identity

The identity internally holds a unique ID (the
[`UniqueIdentity`](https://docs.rs/covidcotra/latest/covidcotra/struct.UniqueIdentity.html)).  This identity is only ever
transmitted to the central authority when the person behind this identity
tests positive.  However also IDs of potential contacts are transmitted.

An identity can be looked at in two other ways: a
[`ShareIdentity`](https://docs.rs/covidcotra/latest/covidcotra/struct.ShareIdentity.html) which is an identity which
is encrypted with the public key of the public authority and is only ever
shared with other devices.  The share ID should rotate regularly and can
be used by the central authorities to determine that a user saw another user.
Since the IDs rotate it's impossible (at least on this level) for a device
to determine that they have seen a device a second time.

Secondarily there is the [`HashedIdentity`](https://docs.rs/covidcotra/latest/covidcotra/struct.HashedIdentity.html).
This is a hashed version of the unique ID which can be used to "poll" for
updates or subscribe to a push channel.  The central authority cannot map
from hashed identity to unique identity but the other way round.  This means
that the unique identity (which eventually gets associated with a real human
identity if revealed) only gets known to the central authority under the
following circumstances:

- a user tests positive and reveals
- the central authority listens on device IDs walking around with a lot of
  devices deployed all over the place.
- a user submits a list of contacts they saw

## Notifications

The central authority marks the hashed IDs of contacts as tainted.  A
device polls alls of the hashed IDs of all the unique IDs it generated and
used in the last N days (for instanc 14 days) for tainted status.  If any
show up as tained they should contact the authorities.

## ID Behavior

* unique ID: you can make multiple but not rotate them too often.  You should
  never share this ID because if someone else has it, they can *become* you.
* share ID: this is good to share with others but you should rotate it regularly
* hashed ID: a derived version of the unique ID that is used for polling for
  tainted status.  You should only send a hashed ID to the authority and nobody
  else otherwise they can poll your taint status.

## ID Cycling

When a user is revealed by testing positive they are encouraged to rotate
the unique ID.  They can do that safely because after testing positive they
will end up in quarantine anyways.  Since other users unique IDs are also
revealed through contact list uploaded they are encouraged to rotate once
they test positive.  If they test negative they could start sending with a
new ID instead of the old one but they would still need to poll the old ID
for updates for an extended period of time.

Generally devices are free to have multiple identities if they want.  For
instance they could chose to transmit not just different share IDs but also
use different unique IDs for extra privacy.  A device could for instance
roll a new ID every day.  The only downside of this is that it then needs
to poll the hashed ID for each generated ID until the infection window
(~14 days?) made an ID expire naturally.

This is a proof of concept [for this blog post about contact
tracing](https://lucumr.pocoo.org/2020/4/3/contact-tracing/).

License: Apache-2.0
