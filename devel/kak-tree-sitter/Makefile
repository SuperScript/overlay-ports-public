PORTNAME=	kak-tree-sitter
DISTVERSION=	1.1.1
PORTREVISION=	1
CATEGORIES=	devel

MAINTAINER=	freebsd@superscript.com
COMMENT=	Integration of tree-sitter with kakoune
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=    ${LOCALBASE}/git:devel/git

USES=		cargo
USE_GITHUB=     yes
GH_ACCOUNT=     superscript
GH_TAGNAME=     4a71775

CARGO_INSTALL_PATH= kak-tree-sitter ktsctl

PLIST_FILES=    bin/kak-tree-sitter \
                bin/ktsctl

.include <bsd.port.mk>

