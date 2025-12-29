PORTNAME=	kak-tree-sitter-languages
DISTVERSION=	v3.1.1
PORTREVISION=	1
CATEGORIES=	devel
MASTER_SITES=   https://git.sr.ht/~hadronized/kak-tree-sitter/archive/:git_sr_ht
DISTFILES=      kak-tree-sitter-${DISTVERSION}${EXTRACT_SUFX}:git_sr_ht

MAINTAINER=	freebsd@superscript.com
COMMENT=	Language implementations for kak-tree-sitter
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

BUILD_DEPENDS=  ktsctl:devel/kak-tree-sitter

# KTSCTL_FETCH=${DISTDIR}/${PORTNAME}-${DISTVERSION}
KTSCTL_CONFIG=$(CWD)
KTSCTL_COMPILE=${WRKSRC}
KTSCTL_INSTALL=${STAGEDIR}${PREFIX}/share
KTSCTL_ETC=${STAGEDIR}${PREFIX}/etc

NO_TEST=    yes

USE_GITHUB= nodefault

.include "Makefile.gh-tuples"

GIT_CMD=    git

Makefile.gh-tuples!
	@printf '%s\n' 'GH_TUPLE+= \' > Makefile.gh-tuples
	@XDG_CONFIG_HOME="$$(pwd)" ktsctl query -a | tail -n +3 | awk '{print$$1}' \
	| XDG_CONFIG_HOME="$$(pwd)" xargs -L1 -I{} ktsctl query '{}' \
	| sed -nE '/^  Source \(git\): https:\/\//!d; /\/git\.sr\.ht\//d; s#^  Source \(git\): https://([^/]+)/([^/]+)/([^/ ]+)/? *\(([^)]+)\).*#\2:\3:\4:ktsctl/sources/\1/\2/\3#p' \
	| sort -u \
	| awk '{lines[++n] = $$0} END {for (i=1; i<=n; i++) {sub(/:ktsctl\//, ":"i"/ktsctl/", lines[i]); printf "\t%s%s\n", lines[i], (i<n?" \\":"")}}' \
	>> Makefile.gh-tuples

post-extract:
	@mkdir -p ${WRKSRC}/ktsctl/sources/git.sr.ht/~hadronized
	@mv ${WRKDIR}/kak-tree-sitter-kak-tree-sitter-${DISTVERSION} ${WRKSRC}/ktsctl/sources/git.sr.ht/~hadronized/kak-tree-sitter

do-build:
	@XDG_CONFIG_HOME=${KTXCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} ktsctl compile --all

do-install:
	@${MKDIR} ${KTSCTL_INSTALL}
	@${MKDIR} ${KTSCTL_ETC}/kak-tree-sitter
	@XDG_CONFIG_HOME=${KTXCTL_CONFIG} XDG_RUNTIME_DIR=${KTSCTL_COMPILE} XDG_DATA_HOME=${KTSCTL_INSTALL} ktsctl install --all

.include <bsd.port.mk>

