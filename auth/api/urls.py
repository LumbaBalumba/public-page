from django.urls import include, path
from rest_framework_simplejwt import views
from rest_framework import routers

from api.views import UserViewSet

router = routers.DefaultRouter()

router.register("user", UserViewSet, basename="user")

urlpatterns = [
    path("", include(router.urls)),
    path("token/", views.TokenObtainPairView.as_view(), name="token_obtain_pair"),
    path("token/verify/", views.TokenVerifyView.as_view(), name="token_verify"),
    path("token/refresh/", views.TokenRefreshView.as_view(), name="token_refresh"),
]
