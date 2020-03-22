# alfred-gitlab-merge-requests

> Alfred workflow to list & search gitlab merge requests (ordered by creation date)

## Download

[Download link](https://github.com/rbaumier/alfred-gitlab-merge-requests/blob/master/alfred-gitlab-merge-requests.alfredworkflow?raw=true)

_Requires the Alfred [Powerpack](https://www.alfredapp.com/powerpack/)._

## Usage

In Alfred, type `gm <optional search filter>`.
The search pattern acts on the following fields (we keep the merge request if any contains the pattern):

- title
- state: "opened", "closed", "locked", or "merged" (type "opened" for example to list open merge requests)
- web_url: allows you to filter by repositories inside your group (because it's included in the url)

Press `<Enter>` to open the repository in your browser.

# What it looks like

![Example](docs/example.gif)

## License

MIT © [Romain Baumier](https://twitter.com/rbaumier)
