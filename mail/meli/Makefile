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

USES=		cargo pkgconfig
USE_GITHUB=	yes
GH_ACCOUNT=	meli

CARGO_FEATURES=	sqlite3 notmuch smtp dbus-notifications gpgme cli-docs jmap

OPTIONS_DEFINE=	DOCS EXAMPLES

# These are the default features from the meli project
# future: add options to enable/disable features as needed

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/meli

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/README.md ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/CHANGELOG.md ${STAGEDIR}${DOCSDIR}

post-install-EXAMPLES-on:
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	${CP} -R ${WRKSRC}/docs/samples/* ${STAGEDIR}${EXAMPLESDIR}

.include <bsd.port.mk>
