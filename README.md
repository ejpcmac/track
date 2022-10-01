# track

A quick-and-dirty CLI tool for tracking parcels with the [La Poste
API](https://developer.laposte.fr/products/suivi/latest).

## Rationale

At the time of writing the first version of this tool, I was in some place with
a really slow internet access. This means refreshing the tracking information
page on [laposte.fr](https://www.laposte.fr/outils/suivre-vos-envois) was taking
minutes. As I wanted to be able to track my parcels with a very low bandwidth
impact, I had the idea of using their public tracking API to do this from my
terminal.

In my browser, I used to keep some tabs open to track incoming parcels. With
`track`, I can now register which parcels to track and get an overview pretty
quickly, even with a high-speed internet access.

## Setup

To use `track`, you need an account on [La Poste
Developer](https://developer.laposte.fr). You can then create a new
application—name it `track` for instance—and register to their [free tracking
API](https://developer.laposte.fr/products/suivi/latest) to get an API key.

Then, install `track`:

```shell
$ cargo install --git https://github.com/ejpcmac/track.git
```

Configure `track` to use your API key:

    $ track init

## Usage

You can track an individual parcel:

    $ track info <tracking_number>

If you want to track a few parcels regularly, you can add them:

    $ track add [tracking_number] [description]

If you call `track add` without parameters, the tracking number and description
will be asked interactively.

To get the status of all tracked parcels:

    $ track all

You can list the tracked parcels:

    $ track list

Or simply remove one from the list:

    $ track remove [tracking_number]

Omitting the tracking number lets you select one from a list.

## Caveats

* There is currently no proper error handling when calling the La Poste API.
* I have tested it only for Colissimo parcels.

## [Contributing](CONTRIBUTING.md)

Before contributing to this project, please read the
[CONTRIBUTING.md](https://github.com/ejpcmac/track/blob/develop/CONTRIBUTING.md).

## License

Copyright © 2020 Jean-Philippe Cugnet

This project is licensed under the [GNU General Public License
3.0](https://www.gnu.org/licenses/gpl-3.0.txt).
