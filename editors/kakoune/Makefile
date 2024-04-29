PORTNAME=	kakoune
PORTVERSION=	sst.1
CATEGORIES=	editors

USE_GITHUB=	yes
GH_ACCOUNT=     mawww
GH_PROJECT=     kakoune
GH_TAGNAME=     e8ad943

MAINTAINER=	ashish@FreeBSD.org
COMMENT=	Modal code editor with a focus on interactivity
WWW=		https://kakoune.org

LICENSE=	UNLICENSE
LICENSE_FILE=	${WRKSRC:H}/UNLICENSE

RUN_DEPENDS=	setsid:sysutils/setsid

USES=		compiler:c++20-lang tar:bzip2
DATADIR=	${PREFIX}/share/kak
DOCSDIR=	${PREFIX}/share/doc/kak
PATCH_WRKSRC=	${WRKSRC:H}
WRKSRC_SUBDIR=	src

OPTIONS_DEFINE=	DEBUG DOCS

DEBUG_MAKE_ARGS=	debug=yes
DEBUG_MAKE_ARGS_OFF=	debug=no

.include <bsd.port.pre.mk>

.if ${OSVERSION} > 1302506
PATCHFILES=	344d31f42b8ced12626d4f87a22ffa5a671668fd.patch
PATCH_SITES=	https://github.com/mawww/kakoune/commit/
PATCH_DIST_STRIP=	-p1
.endif

post-patch:
	@${REINPLACE_CMD} -e '/-O3/d' \
		${WRKSRC:H}/Makefile

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/kak

do-test:
	@cd ${WRKSRC:H}/test && ${SETENV} LC_ALL=en_US.UTF-8 ./run

.include <bsd.port.post.mk>
