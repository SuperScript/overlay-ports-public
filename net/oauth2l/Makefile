PORTNAME=	oauth2l
PORTVERSION=	1.3.3
DISTVERSIONPREFIX=	v
CATEGORIES=	security

MAINTAINER=	freebsd@superscript.com
COMMENT=	OAuth 2.0 command-line utility by Google
WWW=		https://github.com/google/oauth2l

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		go:modules
GO_MODULE=	github.com/google/oauth2l@v${PORTVERSION}
GO_TARGET=	./cmd/oauth2l

PLIST_FILES=	bin/oauth2l

.include <bsd.port.mk>

