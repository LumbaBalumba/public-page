pipeline {
    agent any
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
