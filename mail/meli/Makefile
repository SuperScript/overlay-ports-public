PORTNAME=	meli
PORTVERSION=	0.8.11
DISTVERSIONPREFIX=	v
CATEGORIES=	mail
MASTER_SITES=	https://git.meli-email.org/meli/meli/archive/

MAINTAINER=	freebsd@superscript.org
COMMENT=	Terminal mail client with support for multiple accounts
WWW=		https://meli-email.org/

LICENSE=	GPLv3

LIB_DEPENDS=	libcurl.so:ftp/curl \
		libsqlite3.so:databases/sqlite3 \
		libnotmuch.so:mail/notmuch \
		libgpgme.so:security/gpgme \
		libdbus-1.so:devel/dbus

USES=		cargo pkgconfig perl5 gmake
USE_GITHUB=	yes
GH_ACCOUNT=	meli

CARGO_FEATURES=	sqlite3 notmuch smtp dbus-notifications gpgme cli-docs jmap
CARGO_INSTALL_PATH= meli

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/meli

.include <bsd.port.mk>
