PORTNAME=	pgvector
PORTVERSION=	1.5.0.rc0
DISTVERSIONPREFIX=	v
CATEGORIES=	databases
PKGNAMEPREFIX=	postgresql16-

MAINTAINER=	freebsd@superscript.com
COMMENT=	Graph database extension for PostgreSQL

LICENSE=	APACHE20
LICENSE_FILE=   ${WRKSRC}/LICENSE

BUILD_DEPENDS=	bison:devel/bison
NOT_FOR_ARCHS=		i386

USES=		gmake perl5 pgsql:16
USE_PERL5=	build
WANT_PGSQL=	server

USE_GITHUB=	yes
GH_ACCOUNT=	apache
GH_PROJECT=	age
GH_TAGNAME=	PG16/v${PORTVERSION:S/.rc/-rc/}

MAKE_ARGS=	OPTFLAGS="" \
                BISON=${LOCALBASE}/bin/bison \
                PERL=${LOCALBASE}/bin/perl
MAKE_ENV=	USE_PGXS=1

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/lib/postgresql/age.so

.include <bsd.port.mk>
