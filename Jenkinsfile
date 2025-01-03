pipeline {
    agent any
    environment {
        POSTGRES_USER = credentials('jenkins-i3alumba.ru-auth-postgres-user')
        POSTGRES_PASSWORD = credentials('jenkins-i3alumba.ru-auth-postgres-user')
        POSTGRES_DB = credentials('jenkins-i3alumba.ru-auth-postgres-db')
        DJANGO_SECRET_KEY = credentials('jenkins-i3alumba.ru-auth-django-secret-key')
    }
    stages {
        stage('Stop') {
            steps {
                sh 'echo "Stopping..."'
                sh 'docker compose down'
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
                sh 'docker compose up -d'
            }
        }
    }
}
