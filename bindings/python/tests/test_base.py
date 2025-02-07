from inputtino import _core
from inputtino.base import DeviceDefinition


def test_device_definition_creation():
    definition = DeviceDefinition(
        name="Test Device",
        vendor_id=0x1234,
        product_id=0x5678,
        version=1,
        device_phys="/dev/input/test",
        device_uniq="unique_id",
    )
    assert definition.name == "Test Device"
    assert definition.vendor_id == 0x1234
    assert definition.product_id == 0x5678
    assert definition.version == 1
    assert definition.device_phys == "/dev/input/test"
    assert definition.device_uniq == "unique_id"


def test_device_definition_to_core():
    definition = DeviceDefinition(
        name="Test Device",
        vendor_id=0x1234,
        product_id=0x5678,
        version=1,
        device_phys="/dev/input/test",
        device_uniq="unique_id",
    )
    core_def = definition.to_core()

    assert isinstance(core_def, _core.DeviceDefinition)
    assert core_def.name == definition.name
    assert core_def.vendor_id == definition.vendor_id
    assert core_def.product_id == definition.product_id
    assert core_def.version == definition.version
    assert core_def.device_phys == definition.device_phys
    assert core_def.device_uniq == definition.device_uniq
