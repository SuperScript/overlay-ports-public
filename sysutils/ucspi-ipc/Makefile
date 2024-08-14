PORTNAME=	ucspi-ipc
PORTVERSION=	0.70
PORTREVISION=	1
DISTVERSIONPREFIX=	v
CATEGORIES=	sysutils
MASTER_SITES=	http://www.superscript.com/ucspi-ipc/

MAINTAINER=	freebsd@superscript.com
COMMENT=	UCSPI tools for building local-domain client-server applications

USE_GITHUB=	yes
GH_ACCOUNT=	SuperScript
GH_PROJECT=	ucspi-ipc

USES=	gmake

DOCS=		CHANGES

OPTIONS_DEFINE=	DOCS
PLIST_SUB=	VERSION=${PORTVERSION}

do-configure:
	${ECHO_CMD} "${CC} ${CFLAGS}" > ${WRKSRC}/src/conf-cc
	${ECHO_CMD} "${CC} ${CFLAGS}" > ${WRKSRC}/src/conf-ld

do-build:
	cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} ${MAKE_CMD} -C src it

do-install:
	@${MKDIR} ${STAGEDIR}${PREFIX}/libexec/${PORTNAME}/${PORTVERSION}/command
.for x in ipccat ipcconnect ipcdo ipcrun
	${INSTALL_SCRIPT} ${WRKSRC}/src/${x} ${STAGEDIR}${PREFIX}/libexec/${PORTNAME}/${PORTVERSION}/command
	${LN} -sf ../libexec/${PORTNAME}/${PORTVERSION}/command/${x} ${STAGEDIR}${PREFIX}/bin/${x}
.endfor
.for x in ipcclient ipccommand connect-io ipcexec ipcexec-config \
	ipcexecrules ipcexecrulescheck ipcrules ipcrulescheck ipcserver
	${INSTALL_PROGRAM} ${WRKSRC}/src/${x} ${STAGEDIR}${PREFIX}/libexec/${PORTNAME}/${PORTVERSION}/command
	${LN} -sf ../libexec/${PORTNAME}/${PORTVERSION}/command/${x} ${STAGEDIR}${PREFIX}/bin/${x}
.endfor

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	cd ${WRKSRC}/src && ${INSTALL_DATA} ${DOCS} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
