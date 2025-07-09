pipeline {
    agent any
    environment {
        POSTGRES_USER = credentials('jenkins-i3alumba.ru-auth-postgres-user')
        POSTGRES_PASSWORD = credentials('jenkins-i3alumba.ru-auth-postgres-user')
        POSTGRES_DB = 'auth'
        DJANGO_SECRET_KEY = credentials('jenkins-i3alumba.ru-auth-django-secret-key')
        MINIO_ROOT_USER = credentials('jenkins-i3alumba.ru-minio-root-username')
        MINIO_ROOT_PASSWORD = credentials('jenkins-i3alumba.ru-minio-root-password')
    }
    stages {
        stage('Stop') {
            steps {
                sh 'echo "Stopping..."'
                sh 'docker compose down --remove-orphans'
            }
        }
        stage('Build') {
            steps {
                sh 'echo "Building..."'
                sh 'docker compose build'
            }
        }
        stage('Deploy') {
            steps {
                sh 'echo "Deploying..."'
                sh 'docker compose up -d --remove-orphans'
            }
        }
    }
}
