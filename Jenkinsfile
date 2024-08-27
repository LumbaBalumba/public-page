pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'echo "Building..."'
                sh 'docker compose build'
            }
        }
        stage('Testing') {
            steps {

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
