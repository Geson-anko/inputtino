import pygame
import pytest


@pytest.fixture
def pygame_window():
    """Setup pygame window for input testing."""
    pygame.init()
    screen = pygame.display.set_mode((400, 400))
    yield screen
    pygame.quit()
