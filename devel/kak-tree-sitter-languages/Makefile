PORTNAME=	kak-tree-sitter-languages
DISTVERSION=	1.1.1
PORTREVISION=	1
CATEGORIES=	devel

MAINTAINER=	freebsd@superscript.com
COMMENT=	Language implementations for kak-tree-sitter
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

FETCH_DEPENDS=    ktsctl:devel/kak-tree-sitter
BUILD_DEPENDS=    ktsctl:devel/kak-tree-sitter

DISTFILES=

do-fetch:
	mkdir -p ${WRKDIR}/xdg_runtime_dir
	chmod 0700 ${WRKDIR}/xdg_runtime_dir
	XDG_RUNTIME_DIR=${WRKDIR}/xdg_runtime_dir ktsctl fetch --all

do-build:
	XDG_RUNTIME_DIR=${WRKDIR}/xdg_runtime_dir ktsctl compile --all
	XDG_RUNTIME_DIR=${WRKDIR}/xdg_runtime_dir XDG_DATA_HOME=${STAGEDIR}/${PREFIX}/share ktsctl install --all

stage:
	@${TOUCH} ${TOUCH_FLAGS} ${STAGE_COOKIE}

.include <bsd.port.mk>

