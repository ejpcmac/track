# track-cli

A quick-and-dirty CLI tool for tracking parcels with the [La Poste
API](https://developer.laposte.fr/products/suivi/latest).

# Rationale

At the time of writing this tool, I am in some place with a really slow internet
access. This means refreshing the tracking information page on
[laposte.fr](https://www.laposte.fr/outils/suivre-vos-envois) takes minutes. As
I want to be able to track my parcels with a very low bandwidth impact, I had
the idea of using their public tracking API to do this from my terminal.

In my browser, I used to keep some tabs open to track incoming parcels. With
`track`, I can now register which parcels to track and get an overview pretty
quickly. I am sure it will be helpful even with a high-speed internet access.

# Setup

To use `track`, you need an account on [La Poste
Developer](https://developer.laposte.fr). You can then create a new
application—name it `track-cli` for instance—and register to their [free
tracking API](https://developer.laposte.fr/products/suivi/latest) to get an API
key.

Then, install `track`:

```shell
$ cargo install --git https://github.com/ejpcmac/track-cli.git
```

Configure `track` to use your API key:

    $ track init

# Usage

You can track an individual parcel:

    $ track info <tracking_number>

If you want to track a few parcels regularly, you can add them:

    $ track add <tracking_number> <description>

Then get their status:

    $ track all

You can list the tracked parcels:

    $ track list

Or simply remove one from the list:

    $ track remove <tracking_number>

# Caveats

There is currently no proper error handling, and I have tested it only for
Colissimo.

## License

Copyright © 2020 Jean-Philippe Cugnet

This project is licensed under the [GNU General Public License 3.0](LICENSE).
