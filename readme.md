# European transport feeds

This repository contains the source code of the _List of european transport feeds_ website, which can be found here:

**[https://eu.data.public-transport.earth](https://eu.data.public-transport.earth)**

## Contributing

_Note that, by participating in this project, you commit to the [code of conduct](code-of-conduct.md), and release all contributions under the [ISC license](https://opensource.org/licenses/ISC) (for source code changes) or [to the public domain](https://creativecommons.org/publicdomain/zero/1.0/deed.de) (for changes to the list of feeds), respectively._

You're warmly invited to add or update feeds at any time. You can do so by sending a pull request or leaving us a hint at [the issues page](https://github.com/public-transport/european-transport-feeds/issues). If you want to send a pull request and implement a change yourself, adapt [`feeds.toml`](./feeds.toml) according to the following rules:

- There should only be one feed per country. This feed should also have the scope to (at least eventually) cover the entire country. A counter-example for this would be feeds limited to a specific operator, since such a feed would - by definition - always exclude additional data. Feeds that are not 100% complete but have the scope to cover the whole country eventually are acceptable, though. If some country doesn't have a global feed (yet), you're encouraged to describe the current situation in a new [issue](https://github.com/public-transport/european-transport-feeds/issues).
- If there are some caveats to a feed, please explain them briefly in the `comments` field. Check the german feed for an example.
- Feed URLs shouldn't be too unstable. While it can - unfortunately - be expected that URLs change every couple of months or so, because official providers don't understand stable URLs yet (which is one of the reasons for this repo to exist in the first place), please don't add feeds for which you already know in advance that the URL will break very frequently.
