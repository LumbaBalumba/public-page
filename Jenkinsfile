pipeline {
    agent any
    stages {
        stage('Stop') {
            steps {
                sh 'echo "Stopping..."'
                sh 'docker compose -f ./deploy/compose/compose.yaml down'
            }
        }
        stage('Build') {
            steps {
                sh 'echo "Building..."'
                sh 'docker compose -f ./deploy/compose/compose.yaml build'
            }
        }
        stage('Deploy') {
            steps {
                sh 'echo "Deploying..."'
                sh 'docker compose -f ./deploy/compose/compose.yaml up -d'
            }
        }
    }
}
