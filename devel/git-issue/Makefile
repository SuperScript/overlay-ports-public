# $FreeBSD$

PORTNAME=	git-issue
DISTVERSIONPREFIX=	v
DISTVERSION=    0.1.0_1
CATEGORIES=	devel

MAINTAINER=	freebsd@superscript.com
COMMENT=	Git-based decentralized issue management system
WWW=		https://github.com/dspinellis/git-issue

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

RUN_DEPENDS=	git:devel/git jq:textproc/jq curl:ftp/curl gdate:sysutils/coreutils

USE_GITHUB=	yes
GH_ACCOUNT=	dspinellis
GH_TUPLE=	dspinellis:git-issue:d0569985

NO_BUILD=	yes
NO_ARCH=	yes

# do-install:
# 	${INSTALL_SCRIPT} ${WRKSRC}/git-issue ${STAGEDIR}${PREFIX}/bin
# 	@${MKDIR} ${STAGEDIR}${DATADIR}
# 	cd ${WRKSRC} && ${COPYTREE_SHARE} lib ${STAGEDIR}${DATADIR}

.include <bsd.port.mk>

