import React from 'react';
import styled, { keyframes } from 'styled-components';

const spin = keyframes`
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
`;

const Loader = styled.div`
  border: 16px solid #f3f3f3;
  border-top: 16px solid #3498db;
  border-radius: 50%;
  width: 120px;
  height: 120px;
  animation: ${spin} 2s linear infinite;
`;

const LoadingContainer = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: ${({ theme }) => theme.background}; // Используем цвет фона из темы
  color: ${({ theme }) => theme.text}; // Используем цвет текста из темы
  font-size: 24px;
`;

const LoadingPage = () => {
  return (
    <LoadingContainer>
      <Loader />
      <div>Загрузка...</div>
    </LoadingContainer>
  );
};

export default LoadingPage;