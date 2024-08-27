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
        stage('Run') {
            steps {
                sh 'echo "Running..."'
                sh 'docker compose up -d'
            }
        }
    }
}
