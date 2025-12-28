PORTNAME=	kak-tree-sitter-languages
DISTVERSION=	3.1.1
PORTREVISION=	1
CATEGORIES=	devel

# MASTER_SITES=	github.com:github_com git.sr.ht:git_sr_ht

MAINTAINER=	freebsd@superscript.com
COMMENT=	Language implementations for kak-tree-sitter
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

FETCH_DEPENDS=    ktsctl:devel/kak-tree-sitter
BUILD_DEPENDS=    ktsctl:devel/kak-tree-sitter

DISTFILES=

KTSCTL_FETCH=${DISTDIR}/${DISTNAME}/ktsctl
KTSCTL_COMPILE=${WRKSRC}
KTSCTL_INSTALL=${STAGEDIR}${PREFIX}/share
KTSCTL_ETC=${STAGEDIR}${PREFIX}/etc

NO_TEST=	yes

GIT_CMD=    git

# Test UID to avoid git stupidity when running as an extract dependency
do-fetch:
	@${MKDIR} ${KTSCTL_FETCH}
	@if [ "$$(${ID} -un)" != "nobody" ]; then \
	  XDG_RUNTIME_DIR=${KTSCTL_FETCH} ktsctl fetch --all; \
	fi

do-extract:
	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}

do-build:
	@XDG_RUNTIME_DIR=${KTSCTL_COMPILE} ktsctl compile --all

do-install:
	@${MKDIR} ${KTSCTL_INSTALL}
	@${MKDIR} ${KTSCTL_ETC}/kak-tree-sitter
	@XDG_RUNTIME_DIR=${KTSCTL_COMPILE} XDG_DATA_HOME=${KTSCTL_INSTALL} ktsctl install --all

.include <bsd.port.mk>

