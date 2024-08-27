pipeline {
    agent any
    stages {
        stage('Stop') {
            steps {
                sh 'docker compose down'
            }
        }
        stage('Build') {
            steps {
                sh 'echo "Building..."'
                sh 'docker compose build'
            }
        }
        stage('Runnig') {
            steps {
                sh 'echo "Running..."'
                sh 'docker compose up -d'
            }
        }
    }
}
