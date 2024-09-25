PORTNAME=	kak-tree-sitter-languages
DISTVERSION=	1.1.1
PORTREVISION=	1
CATEGORIES=	devel

# MASTER_SITES=	github.com:github_com git.sr.ht:git_sr_ht

MAINTAINER=	freebsd@superscript.com
COMMENT=	Language implementations for kak-tree-sitter
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

FETCH_DEPENDS=    ktsctl:devel/kak-tree-sitter
BUILD_DEPENDS=    ktsctl:devel/kak_tree_sitter

DISTFILES=

KTSCTL_FETCH=${DISTDIR}/${PORTNAME}
KTSCTL_CONFIG=${WRKSRC}
KTSCTL_COMPILE=${WRKSRC}
KTSCTL_INSTALL=${STAGEDIR}${PREFIX}/share
KTSCTL_ETC=${STAGEDIR}${PREFIX}/etc

NO_BUILD=	yes
NO_TEST=	yes

do-fetch:
	@${MKDIR} ${KTSCTL_FETCH}
	@${CHMOD} 0700 ${KTSCTL_FETCH}
	@XDG_CONFIG=${FILESDIR} XDG_RUNTIME_DIR=${KTSCTL_FETCH} ktsctl fetch --all

do-extract:
	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}

post-patch:
	@${MKDIR} ${KTSCTL_CONFIG}/kak-tree-sitter
	@${CP} ${FILESDIR}/kak-tree-sitter/config.toml ${KTSCTL_CONFIG}/kak-tree-sitter/

do-install:
	@${MKDIR} ${KTSCTL_INSTALL} ${KTSCTL_ETC}/kak-tree-sitter
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} ktsctl compile --all
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} XDG_DATA_HOME=${KTSCTL_INSTALL} ktsctl install --all
	@${INSTALL_DATA} ${KTSCTL_CONFIG}/kak-tree-sitter/config.toml ${KTSCTL_ETC}/kak-tree-sitter/config.toml.sample

stage:
	@${TOUCH} ${TOUCH_FLAGS} ${STAGE_COOKIE}

.include <bsd.port.mk>

