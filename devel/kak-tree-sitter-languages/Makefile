PORTNAME=	kak-tree-sitter-languages
DISTVERSION=	1.1.1
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
KTSCTL_FETCH_CONFIG=${FILESDIR}
KTSCTL_CONFIG=${WRKSRC}
KTSCTL_COMPILE=${WRKSRC}
KTSCTL_INSTALL=${STAGEDIR}${PREFIX}/share
KTSCTL_ETC=${STAGEDIR}${PREFIX}/etc

# NO_BUILD=	yes
NO_TEST=	yes

GIT_CMD=    git

# Test UID to avoid git stupidity when running as an extract dependency
do-fetch:
	@${MKDIR} ${KTSCTL_FETCH}
	@if [ "$$(${ID} -un)" != "nobody" ]; then \
	  XDG_CONFIG_HOME=${KTSCTL_FETCH_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_FETCH} ktsctl fetch --all; \
	fi

	#   for lang in $$(awk -F'[.]' '$$1=="[language" && $$3 == "grammar]"{print $$2};{next}' < files/kak-tree-sitter/config.toml); do \
	#   XDG_CONFIG_HOME=${KTSCTL_FETCH_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_FETCH} ktsctl fetch "$$lang"; \
	# done

	# @if [ ${UID} = 0 ]; then \
	#   @awk -F '[" ]+' '$$1=="url"{url=$$3;next};$$1=="pin"{dir=url;pin=$$3;sub("^[^:/]+://","",dir);printf("%s %s %s\n",url,pin,dir)}' < ${FILESDIR}/kak-tree-sitter/config.toml \
	#     | while read url pin dir; do \
	#         d="${KTSCTL_FETCH}/sources/$${dir}" && test -d "$$d" || ${GIT_CMD} clone "$${url}" --depth 1 "$$d" || exit 1; \
	#         ${GIT_CMD} -C "$$d" rev-parse --quiet --verify "$${pin}^{commit}" >/dev/null || ${GIT_CMD} -C "$$d" fetch --depth 1 origin "$${pin}"; \
	#       done; \
	# fi

do-extract:
	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}

post-patch:
	@${MKDIR} ${KTSCTL_CONFIG}/kak-tree-sitter
	@${CP} ${FILESDIR}/kak-tree-sitter/config.toml ${KTSCTL_CONFIG}/kak-tree-sitter/

do-build:
	@${MKDIR} ${KTSCTL_INSTALL}
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} ktsctl compile --all

do-install:
	@${MKDIR} ${KTSCTL_ETC}/kak-tree-sitter
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} XDG_DATA_HOME=${KTSCTL_INSTALL} ktsctl install --all
	@${INSTALL_DATA} ${KTSCTL_CONFIG}/kak-tree-sitter/config.toml ${KTSCTL_ETC}/kak-tree-sitter/config.toml.sample

# stage:
# 	@${TOUCH} ${TOUCH_FLAGS} ${STAGE_COOKIE}

.include <bsd.port.mk>

