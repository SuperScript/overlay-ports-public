PORTNAME=	kakoune
DISTVERSION=	2024.05.18
PORTREVISION=   2
CATEGORIES=	editors

USE_GITHUB=     yes
GH_ACCOUNT=     superscript
GH_PROJECT=     kakoune
GH_TAGNAME=     d7a7d0f

MAINTAINER=	freebsd@superscript.com
COMMENT=	Modal code editor with a focus on interactivity
WWW=		https://kakoune.org

LICENSE=	UNLICENSE
LICENSE_FILE=	${WRKSRC:H}/UNLICENSE

RUN_DEPENDS=	setsid:sysutils/setsid

USES=		compiler:c++20-lang gmake tar:bzip2
DATADIR=	${PREFIX}/share/kak
DOCSDIR=	${PREFIX}/share/doc/kak

WRKSRC_SUBDIR=	src
PATCH_WRKSRC=	${WRKSRC:H}
BUILD_WRKSRC=	${WRKSRC:H}
INSTALL_WRKSRC=	${WRKSRC:H}
TEST_WRKSRC=	${WRKSRC:H}

OPTIONS_DEFINE=	DEBUG DOCS

DEBUG_MAKE_ARGS=	debug=yes
DEBUG_MAKE_ARGS_OFF=	debug=no

.include <bsd.port.pre.mk>

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/kak

do-test:
	@cd ${TEST_WRKSRC}/test && ${SETENV} LC_ALL=en_US.UTF-8 ./run

.include <bsd.port.post.mk>
