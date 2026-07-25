# overlay-ports-public

## Why it exists

The FreeBSD quarterly pkg repo lags on some packages I need and lacks
others outright. This tree carries the ports I maintain myself, layered
on top of it.

## What it gains

Three things quarterly does not give me: newer versions, packages it
does not carry at all, and builds with options the stock binary packages
do not set.

## Who uses it

Just me, on my own FreeBSD workstations and servers.

## Installation

Nothing to install. A poudriere run pulls this tree straight from GitHub
at build time.

## Day to day

Add or bump a port here, then push. A poudriere bulk build runs against
the tree, and the resulting packages land in my local repo, where my
machines install from.
