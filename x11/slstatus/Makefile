PORTNAME=	slstatus
DISTVERSION=	1.0
PORTREVISION=   4

USE_GITHUB=	yes
GH_ACCOUNT=     superscript
GH_PROJECT=     slstatus
GH_TAGNAME=     b78e4d2

CATEGORIES=	x11
MAINTAINER=	freebsd@superscript.com
COMMENT=	Status monitor for window managers that use WM_NAME or stdin to fill the status bar
WWW=		https://git.suckless.org/slstatus/

LICENSE=	ISCL
LICENSE_FILE=	${WRKSRC}/LICENSE

LIB_DEPENDS=	libsndio.so:audio/sndio

USES=		xorg
USE_XORG=	x11

PLIST_FILES=	bin/slstatus share/man/man1/slstatus.1.gz

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/slstatus ${STAGEDIR}${PREFIX}/bin
	${INSTALL_MAN} ${WRKSRC}/slstatus.1 ${STAGEDIR}${PREFIX}/share/man/man1

.include <bsd.port.mk>
