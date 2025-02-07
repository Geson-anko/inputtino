from dataclasses import dataclass

from . import _core


@dataclass(frozen=True)
class DeviceDefinition:
    """A class representing a virtual device definition.

    This class wraps the C++ DeviceDefinition class and provides a more Pythonic interface.

    Args:
        name: Device name
        vendor_id: Vendor ID for the device
        product_id: Product ID for the device
        version: Device version
        device_phys: Optional physical device path
        device_uniq: Optional unique device identifier
    """

    name: str
    vendor_id: int
    product_id: int
    version: int
    device_phys: str = ""
    device_uniq: str = ""

    def to_core(self) -> _core.DeviceDefinition:
        """Convert to C++ DeviceDefinition object.

        Returns:
            A C++ DeviceDefinition object used by the core library
        """
        definition = _core.DeviceDefinition()
        definition.name = self.name
        definition.vendor_id = self.vendor_id
        definition.product_id = self.product_id
        definition.version = self.version
        definition.device_phys = self.device_phys
        definition.device_uniq = self.device_uniq
        return definition
