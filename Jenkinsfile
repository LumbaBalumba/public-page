pipeline {
    agent any
    stages {
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
