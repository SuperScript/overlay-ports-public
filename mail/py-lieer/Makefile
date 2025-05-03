# $FreeBSD$

PORTNAME=	py-lieer
PORTVERSION=	1.6
CATEGORIES=	mail python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	freebsd@superscript.com
COMMENT=	Fast email-fetching and two-way tag synchronization between notmuch and GMail

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE.md

RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}google-auth-oauthlib>=0:security/py-google-auth-oauthlib@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}google-api-python-client>=0:www/py-google-api-python-client@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}tqdm>=0:misc/py-tqdm@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}notmuch0>=0:mail/py-notmuch2@${PY_FLAVOR} \
		notmuch:mail/notmuch

USES=		python
USE_PYTHON=	autoplist distutils
USE_GITHUB=	yes
GH_ACCOUNT=	gauteh
GH_PROJECT=	lieer
GH_TAGNAME=	v${PORTVERSION}

NO_ARCH=	yes

.include <bsd.port.mk>
