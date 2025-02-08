from django.urls import include, path
from rest_framework import routers
from oauth2_provider import urls as oauth2_urls

from api.views import UserViewSet

router = routers.DefaultRouter()

router.register("user", UserViewSet, basename="user")

urlpatterns = [path("", include(router.urls)), path("o/", include(oauth2_urls))]
