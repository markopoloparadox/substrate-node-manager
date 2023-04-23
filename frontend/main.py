from nicegui import app, ui
import requests
import threading
import time
from datetime import datetime
import json
import nicegui.globals


def start_node():
    api_url = "http://127.0.0.1:3000/start"
    response = requests.get(api_url)
    print(response.text)


def stop_node():
    api_url = "http://127.0.0.1:3000/stop"
    response = requests.get(api_url)
    print(response.text)


start_node_button = ui.button(
    'Start node', on_click=start_node)
stop_node_button = ui.button(
    'STop node', on_click=stop_node)
label = ui.label("Test")


start_node_button.disable()

ui_mem_grid = ui.aggrid({
    'columnDefs': [
        {'headerName': 'Name', 'field': 'name'},
        {'headerName': 'Value', 'field': 'value'},
    ],
    'rowData': [],
    'rowSelection': 'multiple',
}).classes('max-h-40')

ui_disks = ui.aggrid({
    'columnDefs': [
        {'headerName': 'Name', 'field': 'name'},
        {'headerName': 'Total', 'field': 'total'},
        {'headerName': 'Available', 'field': 'available'},
    ],
    'rowData': [],
    'rowSelection': 'multiple',
}).classes('max-h-40')


def mem_val(value) -> int:
    cal = round(int(value) / 1024 / 1024 / 1024, 2)
    return "{} GiB".format(cal)


class MemoryView:
    def __init__(self):
        self.total = '0',
        self.used = '0',
        self.available = '0',
        self.free = '0',

    def update(self, total: int, used: int, available: int, free: int):
        total_f = mem_val(total)
        used_f = mem_val(used)
        available_f = mem_val(available)
        free_f = mem_val(free)
        update = self.total != total_f or self.used != used_f or self.available != available_f or self.free != free_f

        if (not update):
            return

        print("Mem Grid Update")

        # Update self
        self.total = total_f
        self.used = used_f
        self.available = available_f
        self.free = free_f

        # Update Grid
        self.update_grid()

    def update_grid(self):
        ui_mem_grid.options['rowData'].clear()
        ui_mem_grid.options['rowData'] = [
            {'name': 'Total Memory', 'value': self.total},
            {'name': 'Used Memory', 'value': self.used},
            {'name': 'Available Memory', 'value': self.available},
            {'name': 'Free Memory', 'value': self.free},
        ]
        ui_mem_grid.update()


class DisksView:
    def __init__(self):
        self.disks = []
        self.init = False

    def update(self, disks):
        if (not self.init):
            for disk in disks:
                total = mem_val(disk['total'])
                available = mem_val(disk['available'])
                self.disks.append(
                    {'name': disk['name'], 'total': total,
                        'available': available}
                )
            self.update_grid()
            self.init = True
            print("Disk Grid Update")
            return

        update = False

        for i, disk in enumerate(disks):
            total = mem_val(disk['total'])
            available = mem_val(disk['available'])
            if (total != self.disks[i]['total'] or available != self.disks[i]['available']):
                update = True
            self.disks[i]['total'] = total
            self.disks[i]['available'] = available

        if (not update):
            return

        print("Disk Grid Update")

        # Update Grid
        self.update_grid()

    def update_grid(self):
        ui_disks.options['rowData'].clear()
        for disk in self.disks:
            ui_disks.options['rowData'].append(
                {'name': disk['name'], 'total': disk['total'],
                    'available': disk['available']}
            )
        ui_disks.update()


mem_view = MemoryView()
disks_view = DisksView()


def backend_thread():
    api_url = "http://127.0.0.1:3000/"
    response = requests.get(api_url)
    abc = response.json()
    mem = abc['mem']
    mem_view.update(mem['total'], mem['used'], mem['available'], mem['free'])
    disks_view.update(abc['disks'])


async def disconnect() -> None:
    """Disconnect all clients from current running server."""
    for client in nicegui.globals.clients.keys():
        await app.sio.disconnect(client)

app.on_shutdown(disconnect)

ui.timer(1, backend_thread)
ui.run()
