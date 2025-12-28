PORTNAME=	kak-tree-sitter
DISTVERSION=	3.1.1
PORTREVISION=	1
CATEGORIES=	devel

MASTER_SITES=	https://git.sr.ht/~hadronized/${PORTNAME}/archive/
DISTNAME=	${PORTNAME}-v${DISTVERSION}

MAINTAINER=	freebsd@superscript.com
COMMENT=	Integration of tree-sitter with kakoune
WWW=		https://git.sr.ht/~hadronized/kak-tree-sitter

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

RUN_DEPENDS=    git:devel/git

USES=		cargo
WRKSRC=		${WRKDIR}/${PORTNAME}-${DISTNAME}
EXTRACT_ONLY=   ${DISTNAME}${EXTRACT_SUFX}

CARGO_CARGOTOML=        ${WRKSRC}/Cargo.toml
CARGO_INSTALL_PATH= kak-tree-sitter ktsctl

PLIST_FILES=    bin/kak-tree-sitter \
                bin/ktsctl

post-extract:
	:

.include <bsd.port.mk>

