from django.urls import path
from rest_framework_simplejwt import views

from api.views import UserViewSet

urlpatterns = [
    path("token/", views.TokenObtainPairView.as_view(), name="token_obtain_pair"),
    path("token/verify/", views.TokenVerifyView.as_view(), name="token_verify"),
    path("token/refresh/", views.TokenRefreshView.as_view(), name="token_refresh"),
    path("user/", UserViewSet.as_view(), name="user"),
]
