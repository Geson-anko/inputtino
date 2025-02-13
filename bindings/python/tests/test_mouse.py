"""Tests for mouse.py."""

import time
from unittest.mock import MagicMock, patch

import pygame
import pytest

from inputtino._core import Mouse as CoreMouse
from inputtino.mouse import DeviceDefinition, Mouse, MouseButton
from tests.helpers import mark_practical


@pytest.fixture
def mock_core_mouse():
    """Create a mock for CoreMouse with basic functionality."""
    with patch("inputtino._core.Mouse") as mock_mouse_cls:
        mock_instance = MagicMock(spec=CoreMouse)
        mock_mouse_cls.create.return_value = mock_instance
        yield mock_instance


def test_mouse_creation_with_default_device():
    """Test creating a mouse with default device definition."""
    with patch("inputtino._core.Mouse") as mock_mouse_cls:
        Mouse()

        mock_mouse_cls.create.assert_called_once()
        created_def = mock_mouse_cls.create.call_args[0][0]
        assert created_def.name == "Wolf mouse virtual device"
        assert created_def.vendor_id == 0xAB00
        assert created_def.product_id == 0xAB01
        assert created_def.version == 0xAB00


def test_mouse_creation_with_custom_device():
    """Test creating a mouse with custom device definition."""
    custom_def = DeviceDefinition(
        name="Custom Mouse",
        vendor_id=0x1234,
        product_id=0x5678,
        version=0x0001,
        device_phys="/dev/input/test",
        device_uniq="unique_id",
    )

    with patch("inputtino._core.Mouse") as mock_mouse_cls:
        Mouse(custom_def)

        mock_mouse_cls.create.assert_called_once()
        created_def = mock_mouse_cls.create.call_args[0][0]
        assert created_def.name == custom_def.name
        assert created_def.vendor_id == custom_def.vendor_id
        assert created_def.product_id == custom_def.product_id
        assert created_def.version == custom_def.version
        assert created_def.device_phys == custom_def.device_phys
        assert created_def.device_uniq == custom_def.device_uniq


def test_mouse_movement(mock_core_mouse):
    """Test relative mouse movement."""
    mouse = Mouse()
    mouse.move(100, -50)

    mock_core_mouse.move.assert_called_once_with(100, -50)


def test_mouse_absolute_movement(mock_core_mouse):
    """Test absolute mouse movement."""
    mouse = Mouse()
    mouse.move_abs(1920, 1080, 3840, 2160)

    mock_core_mouse.move_abs.assert_called_once_with(1920, 1080, 3840, 2160)


def test_mouse_button_press_and_release(mock_core_mouse):
    """Test mouse button press and release."""
    mouse = Mouse()

    # Test pressing each button
    for button in [
        MouseButton.EXTRA,
        MouseButton.LEFT,
        MouseButton.RIGHT,
        MouseButton.MIDDLE,
        MouseButton.SIDE,
    ]:
        mouse.press(button)
        mock_core_mouse.press.assert_called_with(button)

        mouse.release(button)
        mock_core_mouse.release.assert_called_with(button)


def test_mouse_scrolling(mock_core_mouse):
    """Test mouse scrolling in both directions."""
    mouse = Mouse()

    # Test vertical scrolling
    mouse.scroll_vertical(120)
    mock_core_mouse.vertical_scroll.assert_called_once_with(120)

    # Test horizontal scrolling
    mouse.scroll_horizontal(-120)
    mock_core_mouse.horizontal_scroll.assert_called_once_with(-120)


def test_mouse_nodes(mock_core_mouse):
    """Test getting mouse device nodes."""
    expected_nodes = ["/dev/input/event0", "/dev/input/mouse0"]
    mock_core_mouse.get_nodes.return_value = expected_nodes

    mouse = Mouse()
    assert mouse.nodes == expected_nodes
    mock_core_mouse.get_nodes.assert_called_once()


def test_click(mock_core_mouse):
    """Test single click functionality."""
    mouse = Mouse()
    with patch("time.sleep") as mock_sleep:
        mouse.click(MouseButton.LEFT, 0.5)

        mock_core_mouse.press.assert_called_once_with(MouseButton.LEFT)
        mock_sleep.assert_called_once_with(0.5)
        mock_core_mouse.release.assert_called_once_with(MouseButton.LEFT)


def test_mouse_creation_failure():
    """Test handling of mouse creation failure."""
    with patch("inputtino._core.Mouse") as mock_mouse_cls:
        mock_mouse_cls.create.side_effect = RuntimeError(
            "Failed to create mouse device"
        )

        with pytest.raises(RuntimeError, match="Failed to create mouse device"):
            Mouse()


@mark_practical
def test_practical_mouse_click(pygame_window):
    """Test mouse click is detected by pygame."""
    mouse = Mouse()
    time.sleep(0.1)

    mouse.move_abs(200, 200, 400, 400)
    mouse.click(MouseButton.LEFT)
    time.sleep(0.1)

    click_detected = False
    for event in pygame.event.get():
        if event.type == pygame.MOUSEBUTTONDOWN:
            click_detected = True
            x, y = event.pos
            assert abs(x - 200) <= 5
            assert abs(y - 200) <= 5

    assert click_detected, "Mouse click was not detected"


@mark_practical
def test_practical_mouse_movement(pygame_window):
    """Test mouse movement is detected by pygame."""
    mouse = Mouse()
    time.sleep(0.1)
    mouse.move_abs(100, 100, 400, 400)
    time.sleep(0.1)

    x, y = pygame.mouse.get_pos()
    print(x, y)
    assert abs(x - 100) <= 5
    assert abs(y - 100) <= 5
