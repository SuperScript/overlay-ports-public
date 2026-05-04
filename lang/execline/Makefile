PORTNAME=	execline
DISTVERSION=	2.9.8.1
PORTREVISION=   1
CATEGORIES=	lang shells
MASTER_SITES=	https://www.skarnet.org/software/${PORTNAME}/

MAINTAINER=	freebse@superscript.com
COMMENT=	Lightweight non-interactive sh(1)-like scripting language
WWW=		https://www.skarnet.org/software/execline/

LICENSE=	ISCL
LICENSE_FILE=	${WRKSRC}/COPYING

BUILD_DEPENDS=	skalibs>=2.14.5.1:devel/skalibs
LIB_DEPENDS=	libskarnet.so:devel/skalibs

USES=		gmake
USE_LDCONFIG=	yes
HAS_CONFIGURE=	yes
CONFIGURE_ARGS=	--prefix=${PREFIX} \
		--with-sysdeps=${LOCALBASE}/lib/skalibs/sysdeps \
		--with-lib=${LOCALBASE}/lib \
		--with-dynlib=${LOCALBASE}/lib \
		--with-include=${LOCALBASE}/include \
		--enable-shared

OPTIONS_DEFINE=	DOCS

DOCS=		AUTHORS README doc/*.html

post-install:
	${MV} ${STAGEDIR}${PREFIX}/bin/cd ${STAGEDIR}${PREFIX}/bin/execline-cd
	${MV} ${STAGEDIR}${PREFIX}/bin/wait ${STAGEDIR}${PREFIX}/bin/execline-wait
	${MV} ${STAGEDIR}${PREFIX}/bin/umask ${STAGEDIR}${PREFIX}/bin/execline-umask
	@${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/*
	@${STRIP_CMD} ${STAGEDIR}${PREFIX}/lib/libexecline.so.${PORTVERSION}

post-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	cd ${WRKSRC} && ${INSTALL_DATA} ${DOCS} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>

