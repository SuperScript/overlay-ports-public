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
KTSCTL_CONFIG=${WRKSRC}
KTSCTL_COMPILE=${WRKSRC}
KTSCTL_INSTALL=${STAGEDIR}${PREFIX}/share
KTSCTL_ETC=${STAGEDIR}${PREFIX}/etc

NO_BUILD=	yes
NO_TEST=	yes

GIT_CMD=    GIT_SAVE_DIRECTORIES='*' git
do-fetch:
	@${ECHO_MSG} "do-fetch: $$(id)"
	@${MKDIR} ${KTSCTL_FETCH}
	@${ECHO_MSG} "hereur0:\n$$(ls -lsa ${KTSCTL_FETCH})"
	@awk -F '[" ]+' '$$1=="url"{url=$$3;next};$$1=="pin"{dir=url;pin=$$3;sub("^[^:/]+://","",dir);printf("%s %s %s\n",url,pin,dir)}' < ${FILESDIR}/kak-tree-sitter/config.toml \
	  | while read url pin dir; do \
	      ${ECHO_MSG} "hereur1: fetching to ${KTSCTL_FETCH}/sources/$${dir}"; \
	      d="${KTSCTL_FETCH}/sources/$${dir}" && test -d "$$d" || ${GIT_CMD} clone "$${url}" --depth 1 "$$d" || exit 1; \
	      ${GIT_CMD} -C "$$d" rev-parse --quiet --verify "$${pin}^{commit}" >/dev/null || ${GIT_CMD} -C "$$d" fetch --depth 1 origin "$${pin}"; \
	    done
	@${ECHO_MSG} "hereur2: fetch complete"

# pre-extract:
# 	@${ECHO_MSG} "hereur: pre-extract"

# post-extract:
# 	@${ECHO_MSG} "hereur: post-extract"

# do-extract:
# 	@${ECHO_MSG} "EXTRACT USER: $$(id)"
# 	@${MKDIR} ${KTSCTL_COMPILE}
# 	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
# 	@${CHMOD} -R  ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
# 	@${ECHO_MSG} "DST: ${KTSCTL_COMPILE}:$$(ls -lsa ${KTSCTL_COMPILE})"
# 	@${ECHO_MSG} "DST USER: $$(id)"

# post-extract:
# 	@${ECHO_MSG} "EXTRACT USER: $$(id)"
# 	@${MKDIR} ${KTSCTL_COMPILE}
# 	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
# 	@${CHMOD} -R  ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
# 	@${ECHO_MSG} "DST: ${KTSCTL_COMPILE}:$$(ls -lsa ${KTSCTL_COMPILE})"
# 	@${ECHO_MSG} "DST USER: $$(id)"

post-patch:
	@${ECHO_MSG} "post-patch: $$(id)"
	@${MKDIR} ${KTSCTL_CONFIG}/kak-tree-sitter
	@${CP} ${FILESDIR}/kak-tree-sitter/config.toml ${KTSCTL_CONFIG}/kak-tree-sitter/

do-install:
	@${ECHO_MSG} "do-install: $$(id)"
	@${MKDIR} ${KTSCTL_COMPILE}
	@${CP} -r ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
	@${CHMOD} -R  ${KTSCTL_FETCH} ${KTSCTL_COMPILE}
	@${MKDIR} ${KTSCTL_INSTALL} ${KTSCTL_ETC}/kak-tree-sitter
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} ktsctl compile --all
	@XDG_CONFIG_HOME=${KTSCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} XDG_DATA_HOME=${KTSCTL_INSTALL} ktsctl install --all
	@${INSTALL_DATA} ${KTSCTL_CONFIG}/kak-tree-sitter/config.toml ${KTSCTL_ETC}/kak-tree-sitter/config.toml.sample

stage:
	@${TOUCH} ${TOUCH_FLAGS} ${STAGE_COOKIE}

.include <bsd.port.mk>

